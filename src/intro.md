# Introduction

As software continues to grow more complex, managing that complexity becomes a
key challenge. The Rust programming language is, in my opinion, particularly
promising in this regard. It has language features designed to help developers
handle complexity effectively. Its focus on memory safety and fearless
concurrency allows you to write large, multithreaded applications without
needing to worry about many of the kinds of crashes that can happen in C or
C++.  However, maintaining software projects involves much more than just
writing good code—it also requires structuring projects in a way that supports
long-term growth and sustainability.

Fortunately, the Rust ecosystem provides an useful set of tools that help
automate many aspects of software development and maintenance. These tools are
built on learnings from decades of software engineering, allowing you to manage
projects efficiently and effectively. 

This book is designed to teach you how to take full advantage of Rust's
ecosystem. It will guide you on how to keep your Rust projects free of bugs and
maintainable with less effort, making them easier to use and extend by others.

The book follows a recipe format, with each chapter being self-contained, so
you can focus on specific topics as needed. You don't need to implement every
suggestion in this book. Instead, use it as a source of inspiration to find the
approaches that work best for your project.

## Target Audience

This book is aimed at anyone who wants to start, maintain or collaborate on Rust
software projects.

You can read this book at several levels. If you are a very technical person
with a lot of project experience, you can use this as a recipe book showing you
examples of how to implement various practises in real-world Rust projects. If
you are less technical, but want to understand what is possible in terms of
automation that can lead to higher quality code and save development time, you
can use this book as an overview various strategies and what they accomplish.

Although the focus is specifically on Rust software projects, some of the
information contained in this book is also useful for software projects in
general. It covers various good practises of software development, containing
insights from various companies and successful projects.

## How to read this book

This book is structured like a recipe book: you can read it cover-to-cover,
if you like. But you can also use it as a tool to look up recipes for how
to solve issues you might run across.



<!--

I wrote this book as a guide for how to bootstrap, structure and maintain Rust
projects out of a passion for the Rust programming language. Having accumulated
some opinions on what allows projects to succeed, I feel that it is valuable to
share what I have learned.

You should not view this as a definitive guide, but rather as a collection of
nuggets of advice that come with rationale and examples. Not every piece of
advice may apply to your project, and you might have different opinions than I
have.  Whether you agree with the advice I give in this book or not, I hope
that you can draw some benefit from it and find useful patterns.

The Rust programming language is a lot younger than some of the programming
languages you have worked with in the past. It also has a development velocity
that is faster than you are used to. You may find that some pieces of advice
become outdated over time. I do my best to try to update this book when I can,
but you should feel free to do your own research.

If you find something in this book that is ourageously wrong, feel free to
create a [merge request][repo] with your suggestion, I am happy to take a look
at it.

[repo]: https://gitlab.com/rust-project-primer/book

This book is a living document, not a finished product. It will be periodically
updated with new insights and revised examples, reflecting ongoing learnings in
the field of Rust programming.

-->
