use anyhow::{bail, Result};
use clap::Parser;
use mdbook_files::FilesPreprocessor;
use mdbook_preprocessor::{parse_input, Preprocessor};
use options::{Command, Options};
use std::io;

mod options;

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
                let (ctx, book) = parse_input(io::stdin())?;
                let output = preprocessor.run(&ctx, book)?;
                serde_json::to_writer(io::stdout(), &output)?;
                Ok(())
            }
            Some(Command::Install(_command)) => Ok(()),
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let options = options::Options::parse();
    let renderer = FilesPreprocessor::new();
    options.run(&renderer)
}
