---
# Documentation: https://sourcethemes.com/academic/docs/managing-content/

title: "Are You Sure You Want to Use MMAP in Your Database Management System?"
subtitle: "A summary of the 'Are You Sure You Want to Use MMAP in Your Database Management System?' paper"
summary: ""
authors: []
tags: ["papers","summaries"]
categories: ["Computer Science", "Databases"]
date: 2022-01-14T10:49:41+01:00
lastmod: 2022-01-14T23:58:33+01:00
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

This year I've decided to start writing a quick summary of some of the papers and books I read. I don't know how long this will last, but I am going to give it a try (I haven't been reading that much lately anyway).

So, to get myself started I am going to use the first paper I have read this year: [Are You Sure You Want to Use MMAP in Your Database Management System?](http://cidrdb.org/cidr2022/papers/p13-crotty.pdf)

This paper doesn't introduce any particular innovation but it does a really great job putting together many of the different problems that different database systems have found while using MMAP as an alternative to their buffer pool implementations.

## MMAP Overview

The paper starts providing a short introduction to [MMAP](https://man7.org/linux/man-pages/man2/mmap.2.html). MMAP is an abstraction provided by the underlying OS that maps the content of a file that's residing on secondary storage into a program's address space, transparently loading/unloading pages when the program references them. You can imagine how attractive and "simple" this looks like for developers ...

The general workflow to access a file using `mmap` is as follows:

1. Call `mmap` and get a pointer to the memory-mapped file back.
1. The OS reserves a portion of the program's virtual address but no contents are loaded so far.
1. Use the original pointer to accesss the contents of the file.
1. The OS looks for the corresponding page and, since no contents have been loaded, triggers a page fault in order to load into memory the referenced portion of the file
1. The page table is modified accordingly to point to the new physical address
1. The CPU where the call was initiated caches this entry in its translation lookaside buffer (TLB).

Most programming languages allows you to use the `mmap` abstraction in your programs, so, for example, in `Rust` you can do something like:

```rust
use memmap::MmapOptions;
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
    let file = File::open("/tmp/mmap-example.db")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };

    println!("{:?}", &mmap[10..80]);

    Ok(())
}
```

You don't need to worry about how big `/tmp/mmap-example.db` is, the OS will "transparently" load/unload the pages as needed.

There're a few system calls the database writers can use to perform memory-mapped file I/O :

* __mmap__: We've already covered this; the OS maps the file into a program's virtual address space. We can choose to write any change back to the backing file or keep our change private to us.
* __madvise__: We can provide different hints to the OS about our expected data access patterns. When a page fault happens, the OS will perform different actions depending on the provided hint (`MADV_NORMAL`, `MADV_RANDOM`, `MADV_SEQUENTIAL`).
* __mlock__: we can pin pages in memory, making sure that the OS will never evict them (dirty pages can be flushed at any time, tho)
* __msync__: we can explicitly perform the flush of a memory range to the underlying storage.

## The problems

The paper mentions a few databases that have tried to use mmap somehow: MongoDB, InfluxDB (here I have experienced the problems myself), SingleStore, LevelDB, ... and presents the most common problems people have run into while using this technique.

### Transactional safety

Since the OS is transparently handling the load/unload of the pages, a particular page can be flushed to the underlying storage at any point in time, no matter what the status of the current transaction is.

Different, and usually complicated, protocols are used to prevent the previous problem:

* __OS Copy-On-Write__: This technique creates to different copies of the database file using `mmap`. One of them is the primary copy while the other is used as a private workspace (open with the `MAP_PRIVATE` flag). With this approach, the database needs to make sure that the updates produced by commited transactions have propagated to the primary copy before letting conflicting transaction to move forward and deal with the growth of the private workspace.

* __User Space Copy-On-Write__: This technique involves a manual process where the modified pages are copied to a separate buffer residing in user space. SQLite, MonetDB, and RavenDB use some variant of this technique.

* __Shadow Paging__: This is used by LMDB, and it maintains separate copies for the primary and the shadowed databases, copying the modified pages from the primary to the shadowed, flushing the changes to the secondary storage and flipping the pointer so now the shadowed database becomes the primary and viceversa.

### I/O Stalls

Accesing any page could result in a unexpected I/O stall because the database cannot really know if the page is in memory or not (triggering a blocking page fault in case it isn't).

Potential solutions can be used to deal with the problem described before:

* Pin pages (`mlock`) which are going to be used in the near future. Sadly there's a limit on the total memory that a process can pin.
* Use `madvise` to hint the OS about the potential access patterns. This is much simpler than the previous alternative, offers less control to the developer and the OS is free to ignore the hints and.

### Error handling

Using mmap makes ensuring data integrity a complicated task: page-level checksums should be performed on every page, mmap writting corrupted pages by pointer errors in memory-unsafe managed languages, ...

### Performance issues

Last but no least, the paper introduces performance as the most significant drawback of mmap's transparent paging management. All the previous issues described before could be overcomed through careful implementations, but `mmap`'s bottlenecks cannot be avoided without an OS-level redesign.

In theory the benefits that `mmap` brings on top of the table are:

* The removal of explicit `read/write` system calls.
* The ability to return pointers.
* Lower memory consumption as the data does not need to be replicated in user space.

The three main bottlenecks the paper identifies are: __page table contention__, __single threaded page eviction__, and __TLB shootdowns__, being the latter the trickiest problem

## Experimental analysis

Trying to back the affirmations presented before, the paper presents an experimental analysis where they empirically try to demonstrate the aforementioned issues. They used [fio](https://fio.readthedocs.io/en/latest/) with `IO_DIRECT` (to bypass the OS page chage) as their baseline and focused in read-only scenarios with two common access patterns, random reads and sequential scans

I am just summarizing their results here but if you want to get deeper into the numbers, the paper includes a bunch of different charts.

### Random reads

In this scenario their baseline showed up that fio could fully saturate the NVMe SSD they were using. On the other hand, `mmap` performed significantlly wrose even in those scenarios where the hint matched the workload's access pattern


### Sequential scans

Again, in this scenario `fio` showed up stable performance and `mmap`'s started to drop once the page cache filled up. A slightly different scenario where they used 10 SSDs with RAID 0 showed up roughly 20x performance difference between `fio` and `mmap`.


## Conclusion

The paper makes the case against the use of `mmap` for file I/O in a DB and presents it in a really accesible way. Even if you don't particularly enjoy reading papers, I think this one is really comprehensible and easy to read.

I personally think that the paper shows up a real and valid set of problems happening in `mmap` based systems but I think that writing your own buffer pool is as easy as presented here

[Here](https://db.cs.cmu.edu/mmap-cidr2022/) you can find a web page with the link to the paper, the corresponding video and the source code for the benchmarks.

I think this is the first time I do something like this after I dropped my Ph.D. studies and I am not sure how long this is going to last, but, in case I keep doing it, I hope I can get better at it. The idea is not only to review papers but anything I read and find interesting.