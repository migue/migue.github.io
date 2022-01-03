---
# Documentation: https://sourcethemes.com/academic/docs/managing-content/
 
title: "Formal methods introduction (I)"
subtitle: "A short introduction to formal methods"
authors: ["admin"]
tags: ["formal methods"]
categories: ["Computer Science", "Programming"]
date: 2020-08-12T21:14:30+02:00
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
 


__Disclaimer__: By no means, I am an expert on this topic; I would say I am a beginner on all this stuff. During my Ph.D. time (which I’ve never finished) I used some of the techniques/tools presented here trying to help me formally verify a few designs I was working on. A second attempt to take benefit of formal methods was when I tried to apply some of them during my time at one of the OSGi Alliance Experts Group, looking to formally validate the design of certain aspects in a couple of specs I was working on at that time (I never managed to make a formal proposal to the group, tho).

I would like to start with a question: __how do we know that a program we are writing is correct or no?__ I guess the most probable answer for many of us would be __testing__. We think about many different input cases for our program as we can, write them down and execute our code hoping we can find as many bugs as possible. Of course, there could be a set of inputs we haven’t thought about which cause or program to fail and/or show unexpected behavior. Under this model, we could probably include some other techniques like peer-reviewing or doing source code static analysis.

Trying to extend the testing approach and make it more reliable we have the __property based testing/fuzzing__ approach. In this case, the general idea is to generate tons of random inputs according to a set of patterns and check that a concrete property holds against all the generated cases, broadening the scope of the set of inputs we are passing to the program under testing.

If we try to expand the previous concepts a little bit we get into the model checking world. In this case, we need to define, in a formal and abstract way, what our program/design is doing and the model checker will try to explore all the potential states that our design might ever enter at the time it checks that certain correctness properties hold on every one of the previous states, providing a counterexample in the cases where this is not true. Under this category, we have tools like [TLA+](https://lamport.azurewebsites.net/tla/tla.html) and [Alloy](https://alloytools.org/).

The next sections of this post will be devoted to the model-checking process.

One of the major drawbacks of the model checking approach is the state-explosion problem (the number of potential states our design might enter is way bigger than the available memory) but we would like to formally verify our design even if the set of the states is “infinite”. At this side of the spectrum, we have the formal proof approach, which, personally, I find a bit intimidating (we will talk about it in a future post). Under this category, we have tools like [Isabelle](https://isabelle.in.tum.de/) or [Coq](https://coq.inria.fr/).

# Model checking

I like to think about formal methods like “the applied maths for modeling and analyzing software or hardware systems”: they aim to establish system correctness with mathematical rigor. Entities like IEC (International Electrotechnical Commission), ESA (European Space Agency) or the NASA (National Aeronautics and Space administration) highly recommend the use of formal verification techniques.

Model-based techniques are based on a formal, precise, and unambiguous definition of the system behavior. In the particular case of model checking, once the model is defined, and using a brute-force approach, the checker tries to explore all the possible system states.

One thing I have found particularly useful is that even before running any kind of verification, trying to accurately describe the model has helped me a lot to discover incompleteness and inconsistencies that are usually found at a much later stage of the design.

## The process

Trying to apply a model checking technique to a particular design could be summarized in the next steps (of course, this is how it works for me):

* __Modeling the system:__ in this phase, we define the model using the language of the model checker, perform a few simulations just to do a quick sanity check, and make sure we’re on the right track and formalize the set of properties that need to be validated.

* __Running our model:__ run the model checker in order to validate if the previous properties we have defined hold true in all the potential states our design might enter

* __Analysis:__ Are the previous properties satisfied? Have they been violated? If the checker finds that a certain property has been violated it will provide a counterexample which will help us to refine our design, model definition, or property. It could be possible that we run out of memory because the set of potential states is humungous and, in this case, we would need to reduce them and try again.

## Strengths and weakness

As with any other tool we usually use, the model checking techniques present a set of strengths and weakness I would like to highlight:

### The main strengths of model checking are:

* It provides a general verification approach we can apply to a wide range of domains.
* Partial verification support: the properties can be checked individually.
* Diagnosis information is provided in case a property is invalidated.
* Sound mathematical underpinning: it’s based on graph algorithm theory, data structures, and logic.

###  The main weakness of model checking are:

* It’s not really suitable for data-intensive applications because data usually range over infinite domains.
* There’s no completeness guarantees; it only checks for stated requirements.
* The verification is applied to the system model, and not the system itself.
* It suffers from the state-space explosion problem (despite all the methods that have been developed lately)

# Conclusion

We usually spend way more time on verification than construction when we are building complex systems (hardware or software). I firmly believe than formal methods offer a huge potential to provide more effective verification mechanisms that provide more robust results and reduce the time spent on the process. In the particular case of model checking, in my experience, it provides an effective technique to expose potential errors in our designs.

I had originally planned to provide a short example explaining how to use Alloy to formally verify a design but I think the post has become long enough so I will write about it in a future entry.

Thanks a lot for reading!
