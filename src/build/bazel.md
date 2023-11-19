# Bazel

Bazel is an open-source port of the Blaze build system used internally at
Google.  While it does have a bit of a steep learning curve for those not used
to it, it does support an incredibly wide variety of languages. 

What Bazel lets you do is mix and match programming languages and easily
have them interoperate. For example, you can have some Rust code that you
build into a C library in one build step, and then consume that library in
some C++ project in another step.

Another common pattern is to build some JavaScript-based frontend and then bake
the output of that into a Rust-based backend.


## Reading

- [Bazel rules_rust](https://github.com/bazelbuild/rules_rust)
