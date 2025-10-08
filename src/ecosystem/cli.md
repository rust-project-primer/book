# Command-Line Interface

Rust is commonly used to write command-line applications. The command-line,
especially on UNIX and Linux systems, is very powerful and a good interface to
build tooling, services, and applications deployed on servers.

Command-line services usually have two requirements: they need to parse the
command-line arguments, and they need to output some data. Depending on The kind
of command-line application, the shape of the data that they output looks
different. Command-line applications usually fall into one of a few categories:

- **Tools** perform a single action, such as `git commit` or `git push`. They
  usually output some result data or an error message. Optionally, they can
  usually also output the data in some machine-readable format, such as JSON or
  CSV. This allows their output to be piped into other tools for further
  processing.
- **Read-evaluate-print loops** (REPLs) are interactive environments that allow
  users to enter commands and receive immediate feedback. Examples include
  `python3`, `irb` or `sqlite3`. These are usually used for debugging or
  interactive development.
- **Services** run in the background and provide a way to interact with them,
  such as `ssh` or `httpd`. They usually output a stream of logs. They are
  usually not started by the user, but rather by a system service (systemd), a
  container runtime (Podman) or by a script.
- **Applications** are standalone programs that perform a specific task, such as
  `htop` or `vim`. They usually have an interactive text-based interface and
  stay active until the user exits them.

Besides the differences in their purpose and output data format, command-line
tools usually have a consistent interface for launching and interacting with
them. This interface typically includes options for specifying input and output
files, using environment variables to control behaviour, and returning a status
code that indicates success or failure.

### Parsing Command-Line Arguments

Command-line arguments are quite standardised. Tools often have subcommands,
flags and positional arguments.

- **Subcommands** allow a single tool to have multiple commands, such as
  `git commit` or `git push`. You only need them if your tool needs to perform
  multiple actions (with different sets of flags and positional arguments).
- **Flags** are used to modify the behavior of a command, such as `-v` or
  `--verbose`. Some flags can take values, such as `--log-level info` or
  `-l info`. Often times it is also possible to set defaults using environment
  variables. For example, specifying `LOG_LEVEL=info` will set the log level to
  _info_ by default, but setting the flag will override it.
- **Positional arguments** are used to specify the input or output of a command.
  For example, `cat main.c`.

Rust has a few crates that allow you to parse command-line arguments. The most
popular ones are:

- Command-Line Argument Parser (CLAP)
- StructOpt
- Argh

One big difference between crates is whether they use a declarative approach,
where you define the command-line interface by writing structs and have the
crate derive the parsing logic from them. This is the approach taken by
StructOpt and Argh. CLAP, on the other hand, uses an imperative approach, where
you build the command-line interface by calling methods on a builder. The
declarative approach is often more concise and easier to read, but the
imperative approach gives you more control over the command-line interface.

If you are unsure which crate to use, the CLAP crate is the most popular crate
for parsing command-line arguments. It allows you to define your parser both
declaratively and imperatively, but the declarative approach is often easier to
get started with.

### CLAP

### StructOpt

### Argh

## Input

If you build command-line interfaces that accept user input interactively,

### Rustyline

### Inquire

## Interactive Interfaces

Many command-line tools just output text or data. This is something you can do
with the `println!` macro. If your tool outputs optional log information, you
can use the `eprintln!` macro, which outputs this on the standard error stream
(this allows it to be redirected).

Long-running services and daemons typically only output logs. In that case, you
can use one of the crates from the [logging](./logging.md) ecosystem.

However, some utilities may also require interactive interfaces. There are Rust
crates that you can use.

https://lib.rs/crates/ratatui

### ratatui

## Reading

https://rust-cli-recommendations.sunshowers.io/cli-parser.html

https://infobytes.guru/articles/rust-cli-clap-guide.html

https://www.naurt.com/blog-posts/naurt-introduction-to-command-line-arguments-in-rust
