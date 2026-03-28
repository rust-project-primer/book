use clap::Parser;
use std::path::PathBuf;

/// Preprocessor for mdBook which renders files from a directory as an interactive widget, with
/// syntax highlighting.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Options {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Parser, Debug)]
pub enum Command {
    /// Check if the renderer is supported.
    Supports(SupportsCommand),
    /// Process a parsed book (default).
    Process,
    /// Install support for mdbook-files into the current mdbook project.
    Install(InstallCommand),
}

#[derive(Parser, Debug)]
pub struct SupportsCommand {
    pub renderer: String,
}

#[derive(Parser, Debug)]
pub struct InstallCommand {
    #[clap(long)]
    pub assets: Option<PathBuf>,
}
