use anyhow::{Result, bail};
use clap::Parser;
use mdbook_preprocessor::Preprocessor;
use mdbook_reading::ReadingPreprocessor;
use std::io;

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
}

#[derive(Parser, Debug)]
pub struct SupportsCommand {
    pub renderer: String,
}

impl Options {
    fn run(&self, preprocessor: &dyn Preprocessor) -> Result<()> {
        match &self.command {
            Some(Command::Supports(command)) => {
                if preprocessor.supports_renderer(&command.renderer)? {
                    Ok(())
                } else {
                    bail!("unknown renderer {}", command.renderer);
                }
            }
            None | Some(Command::Process) => {
                let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;
                let output = preprocessor.run(&ctx, book)?;
                serde_json::to_writer(io::stdout(), &output)?;
                Ok(())
            }
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let options = Options::parse();
    let renderer = ReadingPreprocessor;
    options.run(&renderer)
}
