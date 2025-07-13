# Rust Project Primer

_A Practical Guide on how to Structure and Maintain your Rust Projects_

## Summary

> This guide offers advice on structuring and maintaining Rust projects, drawing
> from popular Rust projects and personal experience. It should not be viewed as
> a definitive guide, but rather as a collection of potential issues you might
> encounter and various ways to address them. Recognizing that some problems
> have multiple effective solutions, this guide presents a range of options. The
> hope is that it equips you with valuable advice, allowing your projects to
> benefit from the learnings of those that came before.

## Implementation

This book uses [mdBook][mdbook], a tool that is commonly used by the Rust
ecosystem to write documentation in the style of books. It makes some
modifications to the default mdBook configuration:

- The default font for text is [Inter][inter], a clean sans-serif typeface.
- The default font for code is [Commit Mono][commit-mono], a clean monospaced
  typeface.
- The default font for diagrams is [Routed Gothic](routed-gothic), a recreation
  of the classic [Gorton font used in technical drawings and machine
  labels][gorton].
- Tables have been modified to have higher rows, and a different color header.
- All diagrams are made with [draw.io][drawio]

In addition to that, there are some mdBook plugins that are used:

- mdbook-admonish
- mdbook-reading
- mdbook-files

## License

[CC BY-NC-SA 4.0](LICENSE.md)

[mdbook]: https://rust-lang.github.io/mdBook
[inter]: https://rsms.me/inter/
[commit-mono]: https://commitmono.com/
[routed-gothic]: https://webonastick.com/fonts/routed-gothic/
[gorton]: https://aresluna.org/the-hardest-working-font-in-manhattan/
[drawio]: https://app.diagrams.net
