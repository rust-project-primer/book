# Architecture

Perhaps the most important property of software is the architecture. While the
implementation of functions can easily be changed or optimized, rearchitecting
software, especially collections of systems, is typically a slow and expensive
endeavour.

Software architecture is important for developers to understand. When joining a
new team or project, the very first thing to figure out is how the system works
on a high level. For developers familiar with the software, it is easy to note
down the high-level architecture, but for people unfamiliar with the code base
it is a slow and error-prone process to wade through the code and try to
understand how everything fits together, how components communicate and how
data travels through each component.

If you do not have time to properly document software, the least you should do
is document the high-level architecture.

## Diagrams

Software architecture can be documented in many different ways. Following a
set process for it is not critical, the important part is that there is some
kind of documentation for it. Usually, the most effective kind of documentation
comes in the form of diagrams.

There are various tools that can be used to draw such diagrams:

- [draw.io](https://draw.io)
- PlantUML
- Mermaid

## Documenting Changes

Another important aspect to software architecture is documenting design
decisions. This helps answer *why* the architecture is chosen the way it is.
Having a process around this also helps collaboration, by giving team members
the opportunity to give feedback on proposed design decisions, to find the best
(or sometimes the least worst) way to achieve an intended outcome.

## Reading

https://github.com/joelparkerhenderson/architecture-decision-record?ref=blog.pragmaticengineer.com

[ARCHITECTURE.md](https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html) by Alex Kladov

*Alex argues in this article for adding a file named `ARCHITECTURE.md` into
software repositories to document the architecture of the code base. He argues
that writing good documentation is hard, and it is not often done. But some
someone starting to work in an unfamiliar codebase, such a document with a
bird's-eye view of the layout of the project is invaluable.*

[Architectural Decision Records](https://adr.github.io/)

*ADRs are a tool to record the reasoning behind architecture changes.*

[More Software Projects need Defenses of Design](https://buttondown.email/hillelwayne/archive/more-software-projects-need-defenses-of-design/) by Hillel Wayne

*Hillel argues that many software projects have some design decisions that
might look strange to an outsider. Many of these design decisions are for
backwards compatibility, performance, inspiration by similar projects or other
reasons that are not immediately obvious. For that reason, projects should have
a document defending their design, giving important context and rationale as to
why the decisions were made.*

[Software Architecture is Overrated, Clear and Simple Design is Underrated](https://blog.pragmaticengineer.com/software-architecture-is-overrated/) by Gergely Orosz

*Gergely explains how software is architected in modern tech companies. He
explains the effectiveness of diagrams in communicating architecture choices,
without the need for formal processes such as UML diagrams. He argues having an
informal, collaborative process to come up with architecture is better than
having decisions be made by a software architect, because it makes it easier to
challenge ideas, and that the most important aspect of good architecture is
simplicity.*
