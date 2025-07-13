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
understand how everything fits together, how components communicate and how data
travels through each component.

If you do not have time to properly document software, the least you should do
is document the high-level architecture.

## Publishing

- Markdown
- mdBook

## Diagrams

It tends to be easier to _show_ architecture rather than to _explain_ it.

There are some useful tools that can be used to draw such diagrams:

- [draw.io](https://draw.io)
- [Excalidraw](https://excalidraw.com/)
- [PlantUML](https://www.plantuml.com/)
- [Mermaid](https://mermaid.js.org/)

### draw.io

### Excalidraw

### PlantUML

### Mermaid

## Documenting Changes

Another important aspect to software architecture is documenting design
decisions. This helps answer _why_ the architecture is chosen the way it is.
Having a process around this also helps collaboration, by giving team members
the opportunity to give feedback on proposed design decisions, to find the best
(or sometimes the least worst) way to achieve an intended outcome.

## Reading

```reading
style: article
title: Architectural Decision Record
url: https://github.com/joelparkerhenderson/architecture-decision-record
author: Joel Parker Henderson
---
Architecture decision record (ADR) examples for software planning, IT
leadership, and template documentation.
```

```reading
style: article
title: ARCHITECTURE.md
url: https://matklad.github.io/2021/02/06/ARCHITECTURE.md.html
author: Alex Kladov
archived: matklad-architecture-md.pdf
---
Alex argues in this article for adding a file named `ARCHITECTURE.md` into
software repositories to document the architecture of the code base. He argues
that writing good documentation is hard, and it is not often done. But some
someone starting to work in an unfamiliar codebase, such a document with a
bird's-eye view of the layout of the project is invaluable.
```

```reading
style: article
title: Architectural Decision Records
url: https://adr.github.io/
author: Various Authors
---
ADRs are a tool to record the reasoning behind architecture changes.
```

```reading
style: article
title: More Software Projects need Defenses of Design
url: https://buttondown.email/hillelwayne/archive/more-software-projects-need-defenses-of-design/
author: Hillel Wayne
archived: hillelwayne-defenses-of-design.pdf
---
Hillel argues that many software projects have some design decisions that
might look strange to an outsider. Many of these design decisions are for
backwards compatibility, performance, inspiration by similar projects or other
reasons that are not immediately obvious. For that reason, projects should have
a document defending their design, giving important context and rationale as to
why the decisions were made.
```

```reading
style: article
title: Software Architecture is Overrated, Clear and Simple Design is Underrated
url: https://blog.pragmaticengineer.com/software-architecture-is-overrated/
author: Gergely Orosz
archived: pragmaticengineer-software-architecture-is-overrated.pdf
---
Gergely explains how software is architected in modern tech companies. He
explains the effectiveness of diagrams in communicating architecture choices,
without the need for formal processes such as UML diagrams. He argues having an
informal, collaborative process to come up with architecture is better than
having decisions be made by a software architect, because it makes it easier to
challenge ideas, and that the most important aspect of good architecture is
simplicity.
```

```reading
style: article
title: Architecture diagrams should be code
url: https://brianmckenna.org/blog/architecture_code
author: Brian McKenna
archived: brianmckenna-architecture-diagrams-should-be-code.pdf
---
Brian explains that different people have different views of the architecture
of a complex system, often influenced by which part of the system they work on.
He argues that architecture diagrams can also quickly go out of sync with
reality, as the system evolves. He argues for writing architecture diagrams as
code, using the C4 model and [PlantUML](./diagrams.md#plantuml), or in his case
a Haskell program which produces PlantUML output. That way, these diagrams can
be kept in version control and updated as part of development.
```

```reading
style: article
title: Effective Design Docs
url: https://mmapped.blog/posts/31-effective-design-docs
author: Roman Kashitsyn
archived: mmapped-effective-design-docs.pdf
---
```

```reading
style: article
title: Design Docs
url: https://www.designdocs.dev/
author: Eraser
---
A curated library of our favorite 1000+ design doc examples and templates from
40+ leading engineering organizations and open source projects.
```
