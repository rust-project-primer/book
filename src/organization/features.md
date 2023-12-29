# Features

Where possible, optional functionality in crates should be exposed using
features. 

As crates get large, this makes it possible to build and run tests on just the
feature that you are currently working on, reducing compile times. It also
makes it possible for consumers of the crate to specify which features they need.

Finally, it allows for building zero-cost abstractions: you can build extra
features into your code, but the consumer only pays the cost (in terms of
compile time and binary size) when they are turned on.

## Examples

```files
path = "crate_features"
git_ignore = true
```



## Reading

- The Cargo Book: [Features](https://doc.rust-lang.org/cargo/reference/features.html)
- Effective Rust: [Be weary of feature creep](https://www.lurklurk.org/effective-rust/features.html)
