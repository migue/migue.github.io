---
# Documentation: https://sourcethemes.com/academic/docs/managing-content/

title: "2021 in books, papers and talks"
subtitle: "A quick tour along a few of the talks, papers, and books I've read during 2021"
authors: ["admin"]
tags: ["books", "talks", "papers"]
categories: ["Computer Science", "Programming"]
date: 2022-01-04T15:14:30+01:00
lastmod: 2022-01-04T18:11:17+01:00
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

This year I have read/watched very few tech-related things outside of $job but I would like to share with all of you a few of the ones I've enjoyed the most.

As usual, the contents revolve around databases, systems engineering, cloud computing, runtimes, programming languages and low-level programming, so, if you're not interested in any of these topics you can save some good time and stop reading here.

## Tech Talks

### How we build Firebolt

A really nice talk about how the Firebolt folks built their cloud analytical database; the talk covers different aspects of their architecture: how they used ClickHouse as their starting point, compute and storage separation, their metadata layer (based on [FoundationDB](https://github.com/apple/foundationdb)) among a few other interesting topics.

[Video](https://www.youtube.com/watch?v=9rW9uEJ15tU&t=1s)


### What's the cost of a millisecond?

This talk goes deep in how a single millisecond can affect the behaviour of your service, covering the different aspects of latency amplification.

[Video](https://www.youtube.com/watch?v=JgrcaK0WQCQ)


### Hardware/Software Co-design: The Coming Golden Age

I usually enjoy Brian's talks a lot, and this particular one is no exception. A wonderful talk about the past, present, and future of software and its hardware counterparts.

[Video](https://www.youtube.com/watch?v=nY07zWzhyn4)


### Distributed systems showdown

A wonderful talk about the practicalities of formal methods. During this talk the Apache Bookeeper protocol is modeled using TLA+ and Maelstron/Jepsen (in Java) while comparing both approaches.

[Video](https://www.youtube.com/watch?v=sPSPEgz3o9U)


## Books

### Probabilistic Data Structures and Algorithms for Big Data Applications

An introduction for technology practicioners to probabilistic data structures and algoritms. Different topics like hashing, membership, cardinality among many others are covered.

[Book](https://www.amazon.com/Probabilistic-Data-Structures-Algorithms-Applications-ebook-dp-B07MYKTY8W/dp/B07MYKTY8W)


### Handbook of Model Checking

A really comprehensive introduction of the key foundational topics powering formal methods and verification tools. I haven't fully read it but just the chapters that interested me the most (at some point I would like to fully read it, tho).

[Book](https://link.springer.com/book/10.1007/978-3-319-10575-8)

## Papers

### Using lightweight formal methods to validate a key-value storage node in Amazon S3

The paper describes how the S3 team applied formal methods to validate the correctness of ShardStore, the new key-value store that powers S3 under the hood.

[Paper](https://www.amazon.science/publications/using-lightweight-formal-methods-to-validate-a-key-value-storage-node-in-amazon-s3)

### FoundationDB: A Distributed Unbundled Transactional Key-Value Store

This papers covers FoundationDB's underlying architecture and introduces the framework used to model and test the system itself.

[Paper](https://dl.acm.org/doi/10.1145/3448016.3457559)
