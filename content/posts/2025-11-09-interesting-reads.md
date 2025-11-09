---
title: "Some interesting resources"
subtitle: "A collection of random stuff I have found interesting"
summary: ""
authors: []
tags: ["papers","summaries", "books", "talks"]
categories: ["Computer Science", "Databases", "Distributed Systems", "Machine Learning"]
date: 2025-11-09T09:00:00+01:00
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

A small collection of random things I’ve found interesting during the last few weeks. Each section includes a short note about the content (nothing more). You’ll find books, papers, articles, and more, but you’ll have to read them yourself to decide whether they’re interesting or not.

## The Linux Memory Manager

A modern, in-depth exploration of how Linux handles memory. It's an early access book and it isn't finished yet. I have just skimmed through the first chapters but it looks very promising.

[https://nostarch.com/linux-memory-manager]()

## Building a debugger

Another book from No Starch Press. This one is about building a debugger yourself. It's an early access book too.

[https://nostarch.com/building-a-debugger]()

## Low-Latency Transaction Scheduling via Userspace Interrupts

A paper introducing a new database engine that leverages recent userspace interrupts available in modern CPUs to enable efficient preemptive scheduling. Even if you are not interested in databases, the paper provides a good overview about how interrupts work in modern CPUs.

[https://www.cs.sfu.ca/~tzwang/preemptdb.pdf]()

## Machine Learning Systems. Principles and Practices of Engineering Artificially Intelligent Systems

A comprehensive (~2600 pages) book on building machine learning systems. The book covers the gap between theorical foundations and practical engineering. Of course, I haven't read the whole book, but I browsed some random chapters and it looks like a great resource.

[https://www.mlsysbook.ai/]()

## A generalation of consensus protocols

A blog series about consensus protocols. The author tries to to generalize consensus protocols, proposing a clear and approachable definition of consensus, setting a set of concrete requirements and providing algorithms and approaches that satisfy them. I think it's a great resource for anyone interested in distributed systems and consensus protocols from a more practical perspective.

[https://multigres.com/consensus]()

## Vortex: LLVM for file formats

A nice talk introducing Vortex, a next-generation columnar file format and toolkit designed for high-performance data processing. If you are interested in learning more about file formats and data processing, this talk is worth watching. You can find more detailed information in their [GitHub repository](https://github.com/vortex-data/vortex).

[https://www.youtube.com/watch?v=zyn_T5uragA]()

## F3: The Open-Source Data File Format for the Future

A proposal to build a next-generation open-source file format with interoperability, extensibility, and efficiency as its core design principles. It aims to address the limitations of existing file formats and, at the same time, create a future-proof solution via embedded Wasm decoders. The paper provides a good overview of the design principles and goals of the format and you can find more information in their [GitHub repository](https://github.com/future-file-format/F3)

[https://db.cs.cmu.edu/papers/2025/zeng-sigmod2025.pdf]()
