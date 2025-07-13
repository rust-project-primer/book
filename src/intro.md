# Introduction

Rust as a langauage is magnificent in many ways. While it has a steep learning
curve that can make it difficult to get started, it will give you superpowers
once you are familiar with it. You can suddenly write fast, heavily
multithreaded code that would previously require a team of very senior
developers writing thorough documentation on which locks are needed to access
what, and in which order they need to be acquired to make sure it doesn't crash.
You can write bespoke data structures, knowing there aren't any odd edge-cases
that would make it unsafe to use. You can safely work with untrusted data,
knowing well that you can't accidentally forget a length check, which leads to a
stack overflow and remote-code execution in your production environment. All of
these properties mean that Rust is very scalable: your code bases are not a
house of cards, waiting to collapse. For the most part, if your code compiles,
you know that it works.

Rust has other properties that make it quite interesting. It is somewhat unique
amongst systems programming languages in that you can deploy on a vast breadth
of environments: from lower-power microcontrollers with kilobytes worth of RAM,
to large servers, and even write frontend applications that run in the browser.
The applications are endless, and the ecosystem is ever evolving to make this
easy.

At the same time, this power can be frightning. Once your have learned the
basics, where do you go from here? What parts of the ecosystem do you use for
what? What are some common issues that your project might run into, and how do
you solve them? How do you structure your project, what are some common pitfalls
that you need to avoid?

The idea of this book is to aggregate information and advice that you can use on
your Rust journey. In some ways, it is the book I wish I had read when I got
started with Rust. Knowing the language is one thing, but knowing the ecosystem
is what lets you be productive. Understanding the tools that exist, and when you
should use them. Knowing how you can deploy them. Structuring projects in a way
that supports long-term growth and sustainability.

This book will not teach you Rust, nor will it explain Rust syntax in any way.
For that, there are already plenty of other books, some of which are linked in
the [Prerequisites](intro/prerequisites.md) section. Instead, the focus is on
practises, high-level advice with examples. Ideally, this book should help you,
no matter if you are a project manager evaluating Rust and trying to understand
best practises, if you have recently learned Rust and want to embark on your
first real-world project, or if you already have some experience but want to
lookup specific tooling or solve specific problems.

The book largely follows a recipe format. Each chapter is fairly self-contained,
so you can focus on specific topics as needed. It is not intended as a guide for
you to implement every single piece of advice, more to give an overview of what
exists, how it helps you and when you should use it. To solve specific problems,
once they occur. Use it as a source of inspiration to find the approaches that
work best for your project.

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

This book is structured like a recipe book: you can read it cover-to-cover, if
you like. But you can also use it as a tool to look up recipes for how to solve
issues you might run across.

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
