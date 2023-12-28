# Introduction

```admonish info
This book is still under development. You may find some sections which are
missing, or not fully written out. If you find sections which are incorrect,
feel free to send a correction in the form of an issue or pull request.
```

This guide offers advice on structuring and maintaining Rust projects, drawing
from popular Rust projects and personal experience. It should not be viewed as
a definitive guide, but rather as a collection of potential issues you might
encounter and various ways to address them. Recognizing that some problems have
multiple effective solutions, this guide presents a range of options. The hope
is that it equips you with valuable advice, allowing your projects to benefit
from the learnings of those that came before. 

This book is a living document, not a finished product. It will be periodically
updated with new insights and revised examples, reflecting ongoing learnings in
the field of Rust programming.

## Motivation

The Rust programming language is fundamentally designed to prioritize safety,
aiming to thrive in an era where this aspect is integral to software
engineering, not an afterthought. Yet, writing robust code in Rust requires
more than just language features; it also demands robust development processes.
This book aims to showcase best practices in Rust project development, ensuring
the ecosystem remains as robust as possible.

As Rust gains popularity, many projects are evolving from small, individual or
small-team efforts to large-scale projects with numerous contributors. These
larger projects face different challenges compared to their smaller
counterparts.  This guide aims to facilitate the sharing of advice and
strategies across projects, ensuring that Rust crates maintain their renowned
robustness and stability.

