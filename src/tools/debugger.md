# Debugging

Rust's type system and ownership model prevent entire classes of bugs at compile
time, which means you reach for a debugger less often than in C or C++. But when
you do need one — stepping through unfamiliar code, inspecting a crash dump, or
tracking down a logic error that tests haven't caught — the tooling is there.
This chapter covers binary debuggers, editor integration, record-and-replay
debugging, and async-specific diagnostics with Tokio Console.

## Binary Debuggers

Rust ships with `rust-gdb` and `rust-lldb`, thin wrappers around [GDB][gdb] and
[LLDB][lldb] that add Rust-aware pretty-printers for standard library types.
Without these wrappers, inspecting a `Vec<String>` or `HashMap` in a debugger
shows raw pointer arithmetic and struct internals; with them, you see the
logical contents. Apart from the pretty-printers, they are identical to the
underlying tools.

### `rust-gdb`

GDB is the GNU debugger, available on Linux and most Unix-like systems. To debug
a Rust binary, build in debug mode (the default for `cargo build`) and launch it
under `rust-gdb`:

```bash
cargo build
rust-gdb target/debug/my-app
```

From the GDB prompt, the core commands are `break` to set a breakpoint (by
function name or `file:line`), `run` to start the program, `next` and `step` to
advance by line or into function calls, `print` to inspect variables, and
`backtrace` to see the call stack. GDB also supports conditional breakpoints and
watchpoints (break when a memory location changes).

For post-mortem debugging, you can load a core dump with
`rust-gdb target/debug/my-app core` and inspect the program state at the time of
the crash.

### `rust-lldb`

LLDB is the debugger from the LLVM project, and the default on macOS (where it
ships with Xcode). The workflow is similar:

```bash
cargo build
rust-lldb target/debug/my-app
```

The commands differ slightly from GDB — `breakpoint set` instead of `break`,
`thread backtrace` instead of `backtrace` — but the concepts are the same. LLDB
tends to have better support for macOS-specific features like debugging
universal binaries and Mach-O executables.

### Debug Information

Both debuggers rely on debug information embedded in the binary. Rust includes
full debug info in the `dev` profile by default (`debuginfo = 2`). Release
builds strip it, so if you need to debug an optimized build, set `debug = true`
in `[profile.release]` in your `Cargo.toml`. Be aware that optimizations can
make stepping through code less predictable, as the compiler may reorder or
inline functions.

A common pattern for shipping optimized and stripped binaries, but retaining the
ability to run debuggers against them, is to split the debug information and
keep it, so that when you do have to run a debugger you can download and use it,
but you don't have to ship binaries with debug info. Rust has an option for this
as well:

```toml
[profile.release]
strip = "split"
```

## Editor Integration

VS Code and Zed both provide graphical debugger interfaces that use GDB or LLDB
under the hood. These give you the same capabilities — breakpoints, variable
inspection, call stacks — through a visual interface rather than a command line.

In VS Code, debugging is available through the [CodeLLDB][vscode-codelldb]
extension, which supports launch configurations in `.vscode/launch.json` for
different targets and arguments. Zed has a [built-in debugger][zed-debugger]
that works with both GDB and LLDB. Both editors support setting breakpoints by
clicking in the gutter, stepping through code, and inspecting variables inline.

## Record-and-Replay Debugging

[`rr`][rr] is a record-and-replay debugger that captures the entire execution of
a program and lets you replay it deterministically. During replay, you can step
forward and backward through the execution, set breakpoints, and inspect state
at any point. This is particularly valuable for non-deterministic bugs (race
conditions, bugs that only reproduce intermittently) because once you record a
failing run, you can replay it as many times as needed.

`rr` works with Rust out of the box. Record a test run and replay it:

```bash
rr record target/debug/my-app
rr replay  # opens a GDB session with reverse-stepping
```

During replay, GDB's `reverse-next` and `reverse-step` commands let you step
backward through execution — something that is not possible with a normal
debugger. The main limitation is that `rr` only works on Linux and requires
hardware performance counters, so it does not work inside most virtual machines.

## Tokio Console

[Tokio Console][tokio-console] is a diagnostics tool for async Rust programs,
similar to `top` but for async tasks. It connects to a running application and
shows a live view of all spawned tasks: their state (idle, running, scheduled),
poll durations, waker counts, and warnings about potential issues like tasks
that poll for too long.

It works through two components: the `console-subscriber` crate, which
instruments your Tokio runtime as a `tracing` subscriber layer, and the
`tokio-console` CLI, which connects to the application over gRPC.

To set it up, add the subscriber to your application:

```toml
[dependencies]
console-subscriber = "0.4"
tokio = { version = "1", features = ["full", "tracing"] }
```

```rust
fn main() {
    console_subscriber::init();
    // ... rest of your application
}
```

Then build with the `tokio_unstable` cfg flag and run the console:

```bash
RUSTFLAGS="--cfg tokio_unstable" cargo run
tokio-console  # connects to localhost:6669
```

Tokio Console is most useful for diagnosing performance problems in async
applications: tasks that are slow to poll, tasks that are never woken, or
contention patterns that are hard to see from logs alone. For more background on
its design, see the reading section below.

## Embedded Debugging

Debugging embedded Rust typically involves a hardware debug probe (such as a
J-Link or ST-Link) that connects to the microcontroller's debug interface. The
[`probe-rs`][probe-rs] project provides a Rust-native toolchain for this: it
supports flashing firmware, setting breakpoints, and inspecting memory over SWD
or JTAG. It integrates with VS Code through the [probe-rs
extension][probe-rs-vscode] and can also be used from the command line. The
[Embedded](../ecosystem/embedded.md) chapter covers embedded development in more
detail.

## Reading

```reading
style: article
title: "Debugging Rust Applications with GDB"
url: https://blog.logrocket.com/debugging-rust-apps-with-gdb/
author: Esteban Borai
---
Walkthrough of debugging a Rust program with GDB, from setting breakpoints
and inspecting variables to navigating the call stack. Covers the basics
well and includes screenshots of each step.
```

```reading
style: article
title: "Debugging Rust with rust-lldb"
url: https://dev.to/bmatcuk/debugging-rust-with-rust-lldb-j1f
author: Bob Matcuk
---
Covers the equivalent workflow using LLDB instead of GDB: launching
rust-lldb, setting breakpoints, stepping through code, and inspecting
variables. Useful if you are on macOS or prefer LLDB's interface.
```

```reading
style: article
title: "Debugging Support in rustc"
url: https://rustc-dev-guide.rust-lang.org/debugging-support-in-rustc.html
author: Rust Compiler Team
---
Documents how the Rust compiler generates debug information, including
DWARF support, platform-specific handling, and how type layouts are
communicated to debuggers. Reference material for understanding what
rust-gdb and rust-lldb actually see.
```

```reading
style: article
title: "Using Rust with rr"
url: https://gist.github.com/spacejam/15f27007c0b1bcc1d6b4c9169b18868c
author: Tyler Neely
---
Guide to using rr for record-and-replay debugging with Rust. Covers
recording test runs, setting breakpoints and watchpoints during replay,
and Rust-specific tips like configuring GDB with a pretty-printer for
standard library types.
```

```reading
style: article
title: "Road to TurboWish Part 3: Design"
url: https://blog.pnkfx.org/blog/2021/05/03/road-to-turbowish-part-3-design/
author: Felix S. Klock II
---
Felix describes the design of a tool for debugging asynchronous
applications, exploring how to surface task-level information to
developers. This design work informed what eventually became Tokio
Console.
```

[gdb]: https://sourceware.org/gdb/
[lldb]: https://lldb.llvm.org/
[rr]: https://rr-project.org/
[tokio-console]: https://github.com/tokio-rs/console
[probe-rs]: https://probe.rs/
[probe-rs-vscode]:
  https://marketplace.visualstudio.com/items?itemName=probe-rs.probe-rs-debugger
[vscode-codelldb]:
  https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
[zed-debugger]: https://zed.dev/docs/debugger
