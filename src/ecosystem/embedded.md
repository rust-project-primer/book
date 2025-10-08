# Embedded

Embedded development means writing firmware that runs bare-metal on
microcontrollers. This is often needed when building electronics. Modern
computers have operating systems that abstract away hardware details, but
embedded systems typically run directly on the hardware. The goals for embedded
programming are high reliability and predictability, sometimes with real-time
constraints (meaning that the software has to react to events within a specific
time frame, such as controlling a motor or responding to a sensor).

Embedded development works a bit differently compared to regular application
development. Embedded microcontrollers are tiny, they have flash storage (for
storing their firmware) and RAM that is on the order of kilobytes to megabytes,
not large enough for a full operating system.

### What embedded development looks like

Embedded microcontrollers often use simple 32-bit or 8-bit [Instruction Set
Architectures (ISA)][isa]. Rust has good support for ARM-based ISAs like ARMv6
or ARMv7, and also [RISC-V][riscv], but not for 8-bit ISAs or more exotic ones.
Most embedded systems are ARM based these days, so that is fine.

```admonish info
If you are looking for an embedded microcontroller to get started with using
Rust, both the [RP2350][rp2350] and the [STM32][stm32] family of microcontrollers
are well-supported by a lot of frameworks.

[rp2350]: https://www.raspberrypi.com/products/rp2350/
[stm32]: https://www.st.com/en/microcontrollers-microprocessors/stm32-32-bit-arm-cortex-mcus.html
```

Embedded chips have physical electrical pins. Typically, these can be configured
to be used as [General-Purpose Input/Output (GPIO)][gpio] pins, or they can be
configured as a [peripheral][], where one or more pins implement some protocol.
Peripherals allow you to map some of the output pins to internal hardware that
implements a certain protocol, and the hardware will implement part of the
protocol. Common peripherals are:

- [Pulse-Width Modulation (PWM)][pwm] peripherals allow you to quickly toggle a
  digital pin on and off at a specific carrier frequency, and control how long
  it is turned on (the duty cycle). This allows you to approximate an analog
  output, and allows you to control some external devices that take an analog
  input, such as servo motors.
- [Analog-Digital Converter (ADC)][adc] allows you to read analog voltages, for
  example to get a reading from a sensor that has an analog output.
- [Univeral Asynchronous Receiver-Transmitter (UART)][uart] (also commonly just
  called _serial_) is a common protocol used to connect computers to an embedded
  system to read logs from it or control it.
- [I²C][i2c] (also called Two-Wire Interface, or TWI) is a simple interface that
  is used to connect to other chips or sensors over short distances.
- [Serial Peripheral Interface (SPI)][spi] is a three-wire interface that is
  used to connect to other chips or sensors over short distances.
- [Controller Area Network (CAN) Bus][can] is commonly used for longer-distance
  communication, such as connecting multiple systems in an automotive or
  robotics system.
- [Universal Serial Bus (USB)][usb] is commonly used to connect embedded devices
  to computers or phones

To configure these peripherals, most embedded chips use [memory-mapped
registers][mmior]. These are special memory locations (addresses), where writing
certain values configures the hardware to do specific things.

Finally, embedded chips often use interrupts. These can be configured and cause
the chip to jump to a specific address. For example, timers are often
implemented as interrupts, where you configure them to jump to a specific
function when they fire, or peripherals use them to run some code when there is
incoming data (or when they are ready to write more data).

[riscv]: https://riscv.org/
[mmior]: https://doc.rust-lang.org/embedded-book/start/registers.html
[peripheral]: https://doc.rust-lang.org/embedded-book/peripherals/index.html
[gpio]: https://en.wikipedia.org/wiki/General-purpose_input/output
[i2c]: https://en.wikipedia.org/wiki/I%C2%B2C
[spi]: https://en.wikipedia.org/wiki/Serial_Peripheral_Interface
[uart]:
  https://en.wikipedia.org/wiki/Universal_asynchronous_receiver-transmitter
[can]: https://en.wikipedia.org/wiki/CAN_bus
[pwm]: https://en.wikipedia.org/wiki/Pulse-width_modulation
[adc]: https://en.wikipedia.org/wiki/Analog-to-digital_converter
[isa]: https://en.wikipedia.org/wiki/Instruction_set_architecture
[usb]: https://en.wikipedia.org/wiki/USB

### Challenges in embedded development

What makes writing embedded software challenging is that you are often trying to
do multiple things at once (communicate with other chips, sensors, receive
control input). You may also have some real-time constraints, where you have to
react to certain input events in a specific time-frame. But you are not able to
use threads, there is only a single core, and you do not have a [Memory-Control
Unit (MCU)][mcu] (also called Memory-Management Unit, or MMU) that you can use
to prevent threads from inadvertently accessing or overwriting each other's
memory.

There are [Real-Time Operating Systems (RTOS)][rtos] that you can use, which
provide scheduling and task management, or you have to manually implement some
kind of multi-threading or state machine approach to handle concurrent
operations.

Another challenge often encountered is the ability to see what the
microcontroller is doing, often achieved using a debugger or by logging
information to a serial port. The [probe-rs][] project helps here by making it
easy to flash a binary onto the microcontroller and debug it using a debugger.

[probe-rs]: https://probe.rs/
[mcu]: https://en.wikipedia.org/wiki/Memory_controller
[rtos]: https://example.com/

### Using Rust for embedded development

Embedded development is one of the areas where Rust really shines. The ability
to use zero-cost abstractions to write idiomatic code, that still compiles down
to tiny executables that run on underpowered microcontrollers makes for a
pleasant development experience. The ecosystem's ability to abstract hardware
makes it possible to easily retarget firmware for different microcontrollers,
something which is usually not as easy when writing in C.

Besides the obvious memory-safety and thread-safety benefits of using Rust, it
has some facilities that you can use to express constraints of the hardware and
allow the computer to check that you code is correct (for example the type and
ownership system), and to write useful code to do multiple things at once
without using threads (the async support). There are some frameworks that you
can use to write firmware in Rust that can take care of:

- **Peripherals**: Provide abstractions for using and configuring the
  peripherals of the embedded microcontroller. You can use the type system to
  make sure that you are using peripherals correctly (such as limiting them to
  be used with the pins that they support, or ensuring that they are configured
  correctly when you use them).
- **Scheduling**: Provide abstractions to allow you to write tasks and schedule
  them, ensure that you do not have dead-locks. Some frameworks allow you to
  prioritize tasks, so that you can keep real-time constraints.
- **Communicate**: Provide low-level abstractions the tasks to communicate.

### Using other Rust crates

If you build embedded firmware in Rust, you can use many crates from the Rust
ecosystem. However, you have to keep in mind that many of these crates are not
designed with embedded systems in mind, and may not be suitable for use in
embedded firmware. Specifically, on microcontrollers you typically do not have
an operating system, so you can only use crates that work with
[`no_std`][no_std]. Depending on how you setup your project, you may also not
have a memory allocator, meaning that you cannot use dynamic data structures
like `Vec`, `String` or `HashMap`. However, many popular Rust crates either
support `no_std` out-of-the-box, or have features that allow you to use them
without a memory allocator (either by disabling a default `std` feature, or
enabling a `no_str` feature)

[no_std]: https://docs.rust-embedded.org/book/intro/no-std.html

### Frameworks

In this section, we will present some popular frameworks in the Rust ecosystem
for writing embedded firmware, and discuss briefly what their benefits (and
potentially drawbacks) are.

If you want to use a framework that is easy to get started with and allows you
to write expressive Rust code, you should consider using [Embassy](#embassy). If
you know what you are doing and you just want access to the raw hardware, you
should consider using [Embedded HAL](#embedded-hal). If you want a framework
that allows you to do multiple things at once but also give you hard guarantees
about not deadlocking, you should look into using
[RTIC](#rtic-real-time-interrupt-driven-concurrency).

If you need more of an operating system, because you need stronger isolation
between tasks, consider using [Tock](#tock) or [Hubris](#hubris), which are
operating systems that provide a higher level of abstraction and isolation, at
the expense of some flexibility and needing more resources.

## Embedded HAL

[Embedded HAL](https://github.com/rust-embedded/embedded-hal) is the Rust
project's attempt at building useful abstractions over several microcontrollers,
such that you can write code (drivers, firmware) that are generic over the
underlying hardware.

Embedded HAL provides fundamental abstractions for hardware access through a set
of traits that define standard interfaces for various peripherals. It forms the
foundation upon which higher-level frameworks like Embassy are built. It is
simple and works well across many platforms. It does not provide built-in async
support, so if you want the microcontroller to do multiple things at the same
time, you'll need to handle scheduling and concurrency yourself. However, this
also means it supports a wider variety of targets.

<center>

![Embedded HAL architecture](embedded-hal-architecture.svg)

</center>

The way Embedded HAL works is quite neat: they use [svd2rust][] to parse SVD
files, which describe the hardware registers and their functions, and generate
Rust code from them. This is called the Peripheral Access Crate (PAC). Then, a
safe abstraction layer is built on top of the PAC, called the Hardware
Abstraction Layer (HAL). The HAL provides a safe and easy-to-use interface for
interacting with the hardware. The HAL crate also implements traits from the
`embedded-hal` crate, this allows you to write code and drivers that are generic
over the underlying hardware.

[svd2rust]: https://crates.io/crates/svd2rust

## Embassy

[Embassy](https://embassy.dev) is one of those projects that makes writing
embedded code feel like magic. It is a framework for building firmware for a
variety of mostly ARM-based microcontrollers.

What makes Embassy special is that it supports Async. The async programming
model maps very well to embedded systems: often times, there are many
simultaneous pieces of code waiting for various events to happen, for example
button presses, timers firing, or data coming in from various ports.

If you were to write firmware manually, you would have the choice of manually
programming timers, writing interrupt handlers and building a giant, complicated
mess, or you would have the choice of using a real-time operating system which
comes with its own headaches.

Embassy uses hand-written Hardware Abstraction Layers. This approach gives
developers more control over the API design and allows for better optimizations.
Embassy implements a layered architecture consisting of:

1. A low-level register access layer
2. A hardware abstraction layer (HAL) providing safe access to peripherals
3. Higher-level device drivers and protocol implementations
4. An async/await runtime specifically designed for resource-constrained
   embedded systems

The async runtime efficiently transforms interrupts into task wakeups, allowing
you to write sequential-looking code that actually runs concurrently without the
overhead of an RTOS.

Embassy lets you write readable and portable code and avoid all of the details
on how to program the hardware in a way to do what you want. For example, a loop
that toggles an LED connected to a pin every 150 milliseconds looks like this:

```rust
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    let mut led = Output::new(pin, Level::Low, OutputDrive::Standard);

    loop {
        led.set_high();
        Timer::after_millis(150).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}
```

What is nice about Embassy is that you don't have to be a seasoned firmware
developer to understand how this works, it reads like regular, blocking code.
But behind the scenes, the executor programs a timer that the microcontroller
has, and registers an interrupt handler that when it fires will resume the
future. Embassy is great if you just want your code to work without worrying
about the underlying hardware details.

## RTIC: Real-Time Interrupt-driven Concurrency

[RTIC](https://rtic.rs/2/book/en/preface.html) is a framework for building
concurrent applications on microcontrollers. Unlike Embassy which uses
async/await for concurrency, RTIC is based on a different approach using
interrupt priorities and message passing between tasks.

RTIC provides static priority-based scheduling, meaning tasks have fixed
priorities assigned at compile time. It leverages Rust's type system to ensure
that shared resources are accessed safely without runtime overhead. The
framework handles the scheduling and dispatching of tasks based on hardware
interrupts, making it particularly well-suited for applications with hard
real-time requirements.

One of RTIC's strengths is its compile-time verification of resource sharing -
the compiler can guarantee that there will be no data races between tasks
accessing shared resources.

## Tock

[Tock](https://github.com/tock/tock) is an operating system for microcontrollers
that is written in Rust and focuses on running mutually untrusted applications.
It's a bit different from the other frameworks in this section, in that it is
not just a framework but an operating system. It uses Rust's type system to
create a hardware abstraction layer that enforces access control policies at
compile time.

Tock has a security-focused architecture that separates the kernel into two
components: a small, trusted core kernel and a collection of less trusted
capsules that implement specific functionality. Applications run in isolated
sandboxes, preventing them from interfering with each other or with the kernel.

This design makes Tock particularly well-suited for scenarios where multiple
applications from different sources need to run on the same hardware, such as
IoT devices or sensor networks where different stakeholders may provide
different parts of the software stack.

## Hubris

[Hubris](https://github.com/oxidecomputer/hubris) is a microkernel operating
system for embedded systems developed by Oxide Computer Company. Unlike more
general-purpose embedded frameworks, Hubris is specifically designed with a
focus on security, reliability, and formal verification.

Hubris uses a strict separation of components with explicit message passing for
communication. This architecture helps prevent bugs in one component from
affecting others. Each component runs in its own address space with restricted
permissions, making the system more resilient against both accidental and
malicious failures.

The system is designed to be statically analyzed and formally verified,
providing strong guarantees about its behavior. It is developed by the Oxide
Computer company, which uses it to write firmware for their products.

## Reading

```reading
style: book
title: Rust Embedded Book
url: https://docs.rust-embedded.org/book/
author: Rust-Embedded Project
---
```

```reading
style: book
title: Embassy Book
url: https://embassy.dev/book/
author: Embassy Project
---
```

```reading
style: article
title: Deploying Rust in Existing Firmware Codebases
url: https://security.googleblog.com/2024/09/deploying-rust-in-existing-firmware.html
author: Ivan Lozano and Dominik Maier
archived: google-security-deploying-rust-in-existing-firmware.pdf
---
```

```reading
style: article
title: Async Rust vs RTOS Showdown
url: https://tweedegolf.nl/en/blog/65/async-rust-vs-rtos-showdown
author: Dion Dokter
archived: tweedegolf-async-rust-vs-rtos-showdown.pdf
---
Dion compares a simple firmware for an STM32F446 ARMv7 microcontroller
```

```reading
style: article
title: Implementing async APIs for microcontroller peripherals
url: https://beaurivage.io/atsamd-hal-async
author: Justin Beaurivage
archived: beaurivage-atsamd-hal-async.pdf
---
```

```reading
style: book
title: Hubris Reference
url: https://oxidecomputer.github.io/hubris/reference/
author: Oxide Computer Company
---
```

```reading
style: book
title: The Tock Book
author: Tock developers
url: https://book.tockos.org/introduction.html
---
```
