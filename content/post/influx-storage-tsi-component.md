---
# Documentation: https://sourcethemes.com/academic/docs/managing-content/
title: "InfluxDB storage subsystem: the TSI files"
subtitle: "A short series around the InfluxDB storage subsystem and its most important components"
authors: ["admin"]
tags: ["databases"]
categories: ["Computer Science", "Programming"]
date: 2020-04-22T07:35:33+02:00
featured: false
draft: false
# Featured image
# To use, add an image named `featured.jpg/png` to your page's folder.
# Focal points: Smart, Center, TopLeft, Top, TopRight, Left, Right, BottomLeft, Bottom, BottomRight.
image:
caption: ""
focal_point: ""
preview_only: false
# Projects (optional).
#   Associate this post with one or more of your projects.
#   Simply enter your project's folder or file name without extension.
#   E.g. `projects = ["internal-project"]` references `content/project/deep-learning/index.md`.
#   Otherwise, set `projects = []`.
projects: []
---
 
Final entry on the InfluxDB storage subsystem series. This time I am going to focus on the, relatively new, indexing mechanism that the InfluxDB folks have built into their storage system.
 
If you just arrived here for the first time, you can find some more details about the other major components of the storage system in the previous blog posts of the series: [intro and WAL component](/post/quick-tour-influx-storage/) and [tsm component](/post/influx-storage-tsm-component/).
 
# The origins
If you recall from the previous posts, the TSM files have an index that allows the database to determine where a certain time series is located when a seriesKey and a time range are provided.
 
However, what happens when a more complex query is executed? If instead of just providing a series key the user wants to perform a __group by operation__ using some of the tags, how does the database make sure that it can resolve the query with acceptable performance?
 
Seems like an inverted index could help the database to solve the problem, right? Previous to the TSI alternative, InfluxDB had an in-memory inverted index which was built at startup time from the data stored in the TSM files. This approach worked reasonably fine but, what happens when the number of different series starts to grow larger and larger? Is the database able to handle millions of different series in a single machine? The setups I’ve been dealing with are usually in between 10-12 million series and, in a single server with 64G of RAM, I have been struggling with the memory limitations of the in-memory inverted index approach.
 
With the previous limitations in mind and the goal to be able to support hundreds of millions of different series in a single machine, the InfluxDB folks decided to build the TSI index as a replacement for the aforementioned in-memory index. This new index aims to remove the upper bound limit set by the memory consumption and tries to enforce that, as the number of different series grow bigger, they have an imperceptible impact on the startup times of the database.
 
So, with the development of this new type of index, InfluxDB looks like two databases in one: the time series store we’ve already covered in the previous post and the new inverted index we are going to cover during the next sections.
 
# The TSI index
 
This new data structure moves the index from memory to disk and then these files are memory-mapped, letting the underlying operating system to manage the cache for them. I am curious about the reasoning behind this decision, because, in general, you can have your own cache system and make your own caching decisions based on high-level access patterns that, combined with modern cache techniques like [TinyLFU](https://arxiv.org/abs/1512.00727) can get you far ahead. Of course, this second alternative is way more complicated than the memory-mapped one, and probably not worth to invest the required time from your development team (I am just guessing here).
 
This new index is very similar to the TSM engine described in the previous post: there’s a write-ahead log similar to the one we described during the first post of the series or a compaction process which is constantly running and merging index into larger files.
 
## Understanding the principal components
 
The principal parts of this new index are:
 
* __Index__: contains the entire index for a single shard.
* __Partition__: contains a sharded partition for a shard
* __LogFile__: contains newly written series as an in-memory index and is persisted as a WAL.
* __IndexFile__: contains an immutable, memory-mapped index built from a LogFile or merged from two contiguous index files.
*__SeriesFile__: contains a set of all the series in the whole database (this file is shared across all the different shards in the database)
 
## The lifecycle of a write operation
 
When a new write comes into the system the next steps happen:
 
* The ids of the series are looked up or added to the __SeriesFile__ in case it doesn’t exist
* The series is added to the index
* The series is added to a WAL file and a few different in-memory indexes. Similar to the behavior described in the TSM entry (this process is similar to the process we already described in the first post).
* The series and name sketches are updated with the series and name values respectively.
* Once the previous log file grows above a certain threshold (the default is 1MB), a new active log file is created and the previous log file is compacted into an IndexFile.
 
__Note__: the previous sketches serve as an estimator of the series cardinality. They are implemented using a probabilistic data structure named [HyperLog++](https://research.google/pubs/pub40671/). The main goal of this data-structure is to estimate the number of different elements in very large sets of data using probabilistic algorithms (probabilistic data-structures are a really nice topic but it’s out of the scope of the post to go deeper on how this internally works).

## The structure of the log file
The log file is a list of [LogEntry](https://github.com/influxdata/influxdb/blob/1.8/tsdb/index/tsi1/log_file.go) objects persisted to disk in sequential order:
 
```go
// LogFile represents an on-disk write-ahead log file.
type LogFile struct {
  mu         sync.RWMutex
  wg         sync.WaitGroup // ref count
  id         int            // file sequence identifier
  data       []byte         // mmap
  file       *os.File       // writer
  w          *bufio.Writer  // buffered writer
  bufferSize int            // The size of the buffer used by the buffered writer
  nosync     bool           // Disables buffer flushing and file syncing. Useful for offline tooling.
  buf        []byte         // marshaling buffer
  keyBuf     []byte
 
  sfile   *tsdb.SeriesFile // series lookup
  size    int64            // tracks current file size
  modTime time.Time        // tracks last time write occurred
 
  // In-memory series existence/tombstone sets.
  seriesIDSet, tombstoneSeriesIDSet *tsdb.SeriesIDSet
 
  // In-memory index.
  mms logMeasurements
 
  // Filepath to the log file.
  path string
}
```
There’re a few different entry types that can be stored in the log: `AddSeries`,`DeleteSeries`,`DeleteMeasurement`, `DeleteTagKey`, `DeleteTagValue`.

## The structure of the TSI file

The `.tsi` files are inmutable files where the data is indexed and persisted to disk and memory mapped. From a high level perspective, the index file has the following sections:

* __TagBlocks__: Index of tag values for a single tag key.
* __MeasurementBlock__: Index of measurements and their tag keys.
* __Trailer__: Offset information for the file and HyperLogLog sketches.

The following snippet shows how the different parts of the the `tsi` index file are read from the data stored in the disk. I have removed a few lines I didn't consider to be interesting, you can find all the details in the [original file](https://github.com/influxdata/influxdb/blob/1.8/tsdb/index/tsi1/index_file.go#L168)

```go
// UnmarshalBinary opens an index from data.
// The byte slice is retained so it must be kept open.
func (f *IndexFile) UnmarshalBinary(data []byte) error {
	// Ensure magic number exists at the beginning.
	if len(data) < len(FileSignature) {
		return io.ErrShortBuffer
	} else if !bytes.Equal(data[:len(FileSignature)], []byte(FileSignature)) {
		return ErrInvalidIndexFile
	}

	// Read index file trailer.
	t, err := ReadIndexFileTrailer(data)
	

	// Slice series sketch data.
	f.sketchData = data[t.SeriesSketch.Offset : t.SeriesSketch.Offset+t.SeriesSketch.Size]
	f.tSketchData = data[t.TombstoneSeriesSketch.Offset : t.TombstoneSeriesSketch.Offset+t.TombstoneSeriesSketch.Size]

	// Slice series set data.
	f.seriesIDSetData = data[t.SeriesIDSet.Offset : t.SeriesIDSet.Offset+t.SeriesIDSet.Size]
	f.tombstoneSeriesIDSetData = data[t.TombstoneSeriesIDSet.Offset : t.TombstoneSeriesIDSet.Offset+t.TombstoneSeriesIDSet.Size]

	// Unmarshal measurement block.
    if err := f.mblk.UnmarshalBinary(data[t.MeasurementBlock.Offset:][:t.MeasurementBlock.Size]); 
    
	// Unmarshal each tag block.
	f.tblks = make(map[string]*TagBlock)
	itr := f.mblk.Iterator()

	for m := itr.Next(); m != nil; m = itr.Next() {
		e := m.(*MeasurementBlockElem)

		// Slice measurement block data.
		buf := data[e.tagBlock.offset:]
		buf = buf[:e.tagBlock.size]

		// Unmarshal measurement block.
		var tblk TagBlock
		if err := tblk.UnmarshalBinary(buf); err != nil {
			return err
		}
		f.tblks[string(e.name)] = &tblk
	}

	// Save reference to entire data block.
	f.data = data

	return nil
}
```

Remember that these files are created from the information stored in the `LogFiles`, thanks to the compaction process.
 
# Wrapping up

In this post, I’ve covered a few details about the new TSI component of the InfluxDB’s storage engine, trying to explain how this new type of index is organized, how it works and what problems try to solve.

This is the last post of the InfluxDB storage series. My goal with this series has been to share with you some of the details of how a database internally works (using Influx as the main vehicle to explain the concepts) and some of the different elements that can be found at many different databases (storage formats, write-ahead logs, indexes, …)

I hope you have enjoyed the whole series (I would like to apologize for my poor English skills). It’s been fun to be back in the blog again and write about something I do enjoy a lot (databases) and, who knows, maybe I will be able to write about any other topic I do enjoy in the near future.

See you on the Internet!