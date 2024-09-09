# Embedded

Embedded development is one of the areas where Rust really shines. The ability
to use zero-cost abstractions to write idiomatic code, that still compiles down
to tiny executables that run on underpowered microcontrollers makes for a
pleasant development experience. The ecosystem's ability to abstract hardware
makes it possible to easily retarget firmware for different microcontrollers,
something which is usually not as easy when writing in C.

![bubble graph of popular embedded crates](/graphics/crate-popularity-embedded.svg)

## [Embassy](https://embassy.dev)

Embassy is one of those projects that makes writing embedded code feel like
magic. It is a framework for building firmware for a variety of mostly ARM-based
microcontrollers.

What makes embassy special is that it supports Async. The async programming 
model maps very well to embedded systems: often times, there are many simultaneous
pieces of code waiting for various events to happen, for example button presses,
timers firing, or data coming in from various ports.

If you were to write firmware manually, you would have the choice of manually
programming timers, writing interrupt handlers and building a giant, complicated
mess, or you would have the choice of using a real-time operating system which comes
with it's own headaches.

Embassy lets you write readable and portable code and avoid all of the details
on how to program the hardware in a way to do what you want. For example, a loop that
toggles an LED connected to a pin every 150 milliseconds looks like this:

```rust
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}
```

You don't have to be a seasoned firmware developer to understand how this works,
it reads like regular, blocking code. But behind the scenes, the executor programs
a timer that the microcontroller has, and registers an interrupt handle that
when it fires will resume the future.

## [Embedded HAL](https://github.com/rust-embedded/embedded-hal)

Embedded-HAL is the Rust project's attempt at building useful abstractions over
several microcontrollers, such that you can write code (drivers, firmware) that
are generic over the underlying hardware.

If Embassy is Tokio, then Embedded HAL is the standard library. It is simple,
it works well. It does not support async, so if you want the microcontroller to
do multiple things at the same time, you have to handle it yourself. But at the
same time, it supports a wider variety of targets.

It is even possible to mix Embedded HAL and Embassy to some extent.

## Reading

[Rust Embedded Book](https://docs.rust-embedded.org/book/)

[Embassy Books](https://embassy.dev/book/)

[Deploying Rust in Existing Firmware Codebases](https://security.googleblog.com/2024/09/deploying-rust-in-existing-firmware.html) by Ivan Lozano and Dominik Maier


