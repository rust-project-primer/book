# Preface

I have always been a bit of a programming language nerd. Growing up, I realized
that we live in some of the most exciting time to be alive, the digital
revolution is in full force. In the span of a century, we have fundamentally
changed the way we operate and communicate, by bringing computers into our
daily lives.

Every programming language is a product of its time. Early programming
languages existed in much more resource-constrained environments. They had to
be designed so that they could be compiled without needing to use too much
computing power or memory.

But now, well into the 21st century, we live in an overabundance of computing
resources. Yet, we still continue using the same languages that we came up with
50 years ago. And we can feel the pain: the applications we use are often
either insecure or slow.

In my mind, Rust is a bit of fresh air in the programming language world.  It
is unique in being one of the few languages that manages to pack revolutionary
ideas (memory safety in a systems language, borrow checker) into a language
that is usable in the real world. Previous attempts at adding safety have
typically ended up as language that are neat from an academic viewpoint but not
usable in practise. But now, even Microsoft and Google are adopting it.

To me, Rust makes programming very joyful. It is like LEGO, you have all these
little pieces and you can put them together any way you want. You can write
multithreaded code with confidence. You can write async code with confidence.
You can mix and match. And unlike C and C++, you can have confidence that they
actually work, and that your application doesn't collapse like a Jenga tower
once it gets too big.

Rust certainly isn't perfect, but in my opinion it is *fun*. And I would like
more people to be able to enjoy it. In this book, I try to compress all of the
things I have learned from using Rust in the past 8 years, to make sure that
you can build cool things, too.

## License

I'm licensing this book under the *CC BY-NC-SA 4.0* license. Licensing it this
way gives you a lot of freedom to adapt this book and update it, as long as you
do not do so for commercial gain. I hope that it will be useful to some.

If you want to give something back to the Rust community, I suggest you get
involved in the community, for example:

- Helping with the [Rust compiler development][rustc], [RFC process][rfc] or
  joining a [workgroup][governance],
- Helping the Rust [crate ecosystem][crates], by participating in building
  features or fixing bugs,
- Sharing your knowledge through blog posts, guides or tutorials.

[rustc]: https://github.com/rust-lang/rust
[rfc]: https://github.com/rust-lang/rfcs
[governance]: https://www.rust-lang.org/governance
[crates]: https://crates.io

If you are new to the Rust programming language, I recommend you to spend some
time writing documentation for Rust crates that need it. It is a good way to be
exposed to some Rust code and make an impact. Adding good documentation is
usually appreciated and uncontroversial.
