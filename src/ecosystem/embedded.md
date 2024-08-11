# Embedded

Embedded development is one of the areas where Rust really shines. The ability
to use zero-cost abstractions to write idiomatic code, that still compiles down
to tiny executables that run on underpowered microcontrollers makes for a
pleasant development experience. The ecosystem's ability to abstract hardware
makes it possible to easily retarget firmware for different microcontrollers,
something which is usually not as easy when writing in C.

## [Embassy](https://embassy.dev)

Embassy is one of those projects that makes writing embedded code feel like
magic.


## [Embedded HAL](https://github.com/rust-embedded/embedded-hal)

Embedded-HAL is the Rust project's attempt at building useful abstractions over
several microcontrollers, such that you can write code (drivers, firmware) that
are generic over the underlying hardware.



## Reading

[Rust Embedded Book](https://docs.rust-embedded.org/book/)
