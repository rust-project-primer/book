# Zed

[Zed][zed] is a code editor that comes with support for Rust out-of-the-box. It
deserves a special mention because it itself is written in Rust. It is fairly
minimalist, offering limited support for extensions (only themes, grammars and
language servers can be extended). But the advantage is that it requires no
setup, it understands and can work on Rust projects with no configuration.

![zed editor](https://zed.dev/_next/image?url=%2F_next%2Fstatic%2Fmedia%2Flanguage-aware.147d5036.png&w=2048&q=75)

If you just want an editor that you can use to write Rust code, and you only
need features that `rust-analyzer` comes with out of the box, then it is a good
choice. It is also [open-source][].

- screenshots of all features zed has for Rust projects

## Features

## Notes

Notably, the team behind Zed runs a [blog](https://zed.dev/blog) documenting
their experience building a cross-platform code editor in Rust, with deep dives
into challenges they have faced in doing so and how they managed to tackle them.
A lot of the articles there are good reading for anyone who is interested in
Rust, cross-platform development, real-world asynchronous applications and the
like.

[zed]: https://zed.dev/
[open-source]: https://zed.dev/blog/zed-is-now-open-source
