# Embedded

Embedded development means writing firmware that runs bare-metal on
microcontrollers. This is often needed when building electronics. Modern
computers

Embedded development works a bit differently compared to regular application
development. Embedded microcontrollers are tiny, they have flash storage (for
storing their firmware) and RAM that is on the order of kilobytes, not large
enough for an operating system.

### What embedded development looks like

Embedded microcontrollers often use simple 32-bit or 8-bit [Instruction Set
Architectures (ISA)][isa]. Rust has good support for ARM-based ISAs like ARMv6
or ARMv7, and also [RISC-V][riscv], but not for 8-bit ISAs or more excotic ones.
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
protocol. Common peripheras are:

- [Pulse-Width Modulation (PWM)][pwm] peripherals allow you to quickly toggle a
  digital pin on and off at a specific carrier frequency, and control how long
  it is turned on (the duty cycle). This allows you to approximate an analog
  output, and allows you to control some external devices that take an analog
  input, such as servo motors.
- [Analog-Digital Converter (ADC)][adc] allows you to read analog voltages, for
  example to get a reading from a sensor that has an anolog output.
- [Univeral Asynchronous Receiver-Transmitter (UART)][uart] (also commonly just
  called _serial_) is a common protocol used to connect computers to an embedded
  system to read logs from it or control it.
- [I²C][i2c] (also called Two-Write Interface, or TWI) is a simple two-wire
  interface that is used to connect to other chips or sensors over short
  distances.
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
Unit (MCU)][mcu] that you can use to prevent threads from inadvertently
accessing or overwriting each other's memory.

There are [Real-Time Operating Systems (RTOS)][rtos] that you can use, or you
have to manually implement some kind of multi-threading

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

In this section, we will present some popular frameworks in the Rust ecosystem
for writing embedded firmware, and discuss briefly what their benefits (and
potentially drawbacks) are.

## Embedded HAL

[Embedded HAL](https://github.com/rust-embedded/embedded-hal) is the Rust
project's attempt at building useful abstractions over several microcontrollers,
such that you can write code (drivers, firmware) that are generic over the
underlying hardware.

If Embassy is Tokio, then Embedded HAL is the standard library. It is simple, it
works well. It does not support async, so if you want the microcontroller to do
multiple things at the same time, you have to handle it yourself. But at the
same time, it supports a wider variety of targets.

It is even possible to mix Embedded HAL and Embassy to some extent.

## Embassy

[Embassy](https://embassy.dev) is one of those projects that makes writing
embedded code feel like magic. It is a framework for building firmware for a
variety of mostly ARM-based microcontrollers.

What makes embassy special is that it supports Async. The async programming
model maps very well to embedded systems: often times, there are many
simultaneous pieces of code waiting for various events to happen, for example
button presses, timers firing, or data coming in from various ports.

If you were to write firmware manually, you would have the choice of manually
programming timers, writing interrupt handlers and building a giant, complicated
mess, or you would have the choice of using a real-time operating system which
comes with it's own headaches.

Embassy lets you write readable and portable code and avoid all of the details
on how to program the hardware in a way to do what you want. For example, a loop
that toggles an LED connected to a pin every 150 milliseconds looks like this:

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
it reads like regular, blocking code. But behind the scenes, the executor
programs a timer that the microcontroller has, and registers an interrupt handle
that when it fires will resume the future.

## RTIC: Real-Time Interrupt-driven Concurrency

[RTIC](https://rtic.rs/2/book/en/preface.html)

## Tock

[Tock](https://github.com/tock/tock) is an operating system for microcontrollers
that is written in Rust and focusses on running mutually untrusted applications.

## Hubris

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
