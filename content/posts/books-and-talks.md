---
# Documentation: https://sourcethemes.com/academic/docs/managing-content/

title: "Books and Talks"
subtitle: "A short compilation of books and talks I have 'recently' enjoyed"
authors: ["admin"]
tags: ["books", "talks"]
categories: ["Computer Science", "Programming"]
date: 2020-02-03T15:14:30+01:00
lastmod: 2020-02-05T22:25:17+01:00
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

I would like to share with you a bunch of the tech talks I have recently watched or some of the latest books I have enjoyed.

__Disclaimer__: this list is just a set of resources I have gone through more or less "recently" about a few different topics I use to enjoy. It's not my goal to create a "Best Talks/Papers/Whatever List", just wanted to share with you all some of the things I have found interesting lately (all the resources are listed in no specific order).

__Disclaimer 2__: by "lately" I mean the last ~1.5 years, so probably many of the contents linked below will be quite old for most of you (I have slowed down a lot on my work-related stuff, but this is a different story).

If you do like databases, systems engineering, cloud computing, runtimes and low-level programming, this list might contain some pointers to a book/talk you could enjoy.

## Talks

### Let's Talk Locks! by Kavya Joshi

An amazing talk about how locks are used at different places (Linux syscalls, Go programming language) and its performance implications.

Actually, any talk coming from Kavya is usually wonderful.

[Video](https://www.infoq.com/presentations/go-locks/)


### EBtree — Design for a Scheduler and Use (Almost) Everywhere by Andjelko Iharos

This talk goes into the evolution of HAProxy’s internals and how the created the EBTree data structure in order to manage active and suspended tasks within their scheduler (and how they ended up using it almost everywhere)

[Video](https://www.infoq.com/presentations/ebtree-design/)

### Performance Matters by Emery Berger

I simply love this talk. Emery goes through many of the different factors that, potentially, can affect performance on modern hardware: memory layout, instruction prefetching, branch prediction, ... and some surprising ones like env variables.

During the talk, he presents the __stabilizer__ tool which randomizes programs layouts during runtime and introduced [coz](https://www.usenix.org/node/196222), a causal profiler.

[Video](https://www.youtube.com/watch?v=r-TLSBdHe1A)


### PID Loops and the Art of Keeping Systems Stable by Colm MacCárthaigh

I like Colm's talks a lot. In this case, the talk goes through PID loops, control theory and how a bunch of AWS systems apply these design principles.

[Video](https://www.infoq.com/presentations/pid-loops/)

### Structured Concurrency by Roman Elizarov

Great talk about the evolution of asynchronous APIS in different programming languages and platforms and how they have applied the structured concurrency concepts to the design of the Kotlin's concurrency libraries.

[Video](https://www.youtube.com/watch?v=Mj5P47F6nJg)

### Reduce your storage costs with Transient Replication and Cheap Quorums by Alex Petrov

Alex talks about [Voting With Witnesses](http://www2.cs.uh.edu/~paris/MYPAPERS/Icdcs86.pdf) a replication schema which is used in Google Spanner and Megastore and has inspired Apache Cassandra's Transient Replication and Cheap Quorums implementation

[Video](https://www.youtube.com/watch?v=pEtRBid5oeA)

### ftrace: Where modifying a running kernel all started by Steven Rostedt

If you've ever wondered how tracing works in the Linux kernel you should probably watch it. A highly technical talk but really well executed even an stupid like myself could _understand_ it.

[Video](https://www.youtube.com/watch?v=93uE_kWWQjs)

### Applicable and Achievable Formal Verification by Heidy Khlaaf

Heidy provides a nice overview of a few verification tools and techniques deployed in the safety critical industry and shows how this could be adapted to your systems.

[Video](https://www.youtube.com/watch?v=GIYtzygBgA4)

### Correctness proofs of distributed systems with Isabelle/HOL by  Martin Kleppmann

An extended version of the talk that Martin did at [Strange Loop 2019](https://www.youtube.com/watch?v=7w4KC6i9Yac). He explores how Isabelle can be used to analyze algorithms used in distributed systems, and prove them correct.

[Video](https://www.youtube.com/watch?v=Uav5jWHNghY)

## Books

### Database internals by Alex Petrov

Alex has done an amazing job writing this book. You will find different topics: storage engines like BTrees and LSMs, how data is physically stored and the different building blocks involved, distributed systems and database clustering among many others

If you like databases and its internals this book is going to be a fun read.

[O'Reilly](http://shop.oreilly.com/product/0636920174462.do)

### Serious Cryptography by Jean-Philippe Aumasson

Another wonderful book. A practical guide to modern encryption that goes through the fundamental mathematical concepts at the heart of cryptography: authenticated encryption, hash functions, block ciphers, and public-key techniques (RSA and elliptic curve cryptography).

Totally recommended if you want to learn a little bit more about cryptography, and, even if you're a seasoned practitioner (not my case) I think you can learn a few things.

[O'Reilly](http://shop.oreilly.com/product/9781593278267.do)

### Optimizing Java By Benjamin Evans, James Gough, and Chris Newland

One more book I have enjoyed a lot. A practical approach to JVM performance tuning and how to identify and solve performance related issues. The book will help you to understand Java platform's internals (if you're willing to go through it :) )

[O'Reilly](http://shop.oreilly.com/product/0636920042983.do)

### Efficient IO with io_uring

I wasn't sure where to put this resource; technically this is not a book, nor a paper either. Anyway, this is a gorgeous read about the newest Linux IO interface, io_uring and compare it to the current alternatives.

Why this work is being done, how this works, ... IO_URING is a huge step forward in the Linux kernel and this short introduction provides a gentle introduction

[IO_URING introduction](https://kernel.dk/io_uring.pdf)

### Cloud Native Data Center Networking By Dinesh Dutt

A really nice read about how modern cloud native data centers networks work and the steps required to design a datacenter that's reliable and easy to manage. A mix of theory and practice which tries to guide the reader through everything which is needed to create and operate a network infrastructure in a modern datacenter.

The book covers many different topics: network disaggregation, routing protocols, network virtualization, ...

[O'Reilly](http://shop.oreilly.com/product/0636920217930.do)

### BPF Performance Tools: Linux System and Application Observability by Brendan Gregg

I am still going through it, but I am enjoying it so much I thought I should include it in this list. A really comprehensive guide about BPF tools, performance engineering, and kernel internals. The book includes tons of examples,tools, ...

[O'Reilly](https://www.oreilly.com/library/view/bpf-performance-tools/9780136588870/)

My original idea was to include a section with the papers I have recently enjoyed as well, but the post had become way too long that I decided to split it and do a follow-up including the academic papers.