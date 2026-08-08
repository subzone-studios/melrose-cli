use crate::cli::{Cli, GenerateCommand, GenerateSubcommand};

mod cli;

fn main() {
    match cli::Cli::parse_args() {
        Cli::Generate(shim!(Generate -> Feature {name, context})) => {
            todo!()
        }
        Cli::Generate(shim!(Generate -> Service {..})) => {}
        Cli::Generate(shim!(Generate -> Controller {..})) => {}
    }
}
