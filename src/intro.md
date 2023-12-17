# Introduction

This guide is a collection of advice on how to structure and maintain Rust
projects. Some of this advice is derived from examining how other popular
Rust projects are structured, some is derived from personal experience.

This is by no means a definite guide. Think of it more as a set of issues you
might encounter and suggestions for how they could be solved. Some issues might
have multiple good solutions. The hope is that this guide can set you up with
good advice to implement in your projects, such that they can benefit from
the learnings of projects before yours.

This book is not finished. Instead it is a living document that is occasionally
updated with new learnings and updated examples.

## Motivation

The Rust programming language has tremendous potential to help guide us into
an era where safety is not added as an afterthought. However, writing robust
code requires not only good programming languages, it also requires robust
processes. This book aims to illustrate good practises for Rust projects to
ensure that the ecosystem is as robust as can be.

Additionally, as Rust becomes more popular, there will be many projects that
grow from being small and maintained by one or a handful of people to becoming
popular projects with many contributors. The kinds of challenges faced by 
large projects with hundreds of contributors are vastly different from those
faced by small projects. 

This guide is intended to share advice between projects to make sure that Rust
crates continue to deliver the same robustness and stability that they have
in the past.

