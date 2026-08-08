use clap::{Parser, Subcommand};

use crate::domain::GeneratorContext;

#[derive(Parser)]
#[command(long_about, about, version)]
struct Args {
    #[command(subcommand)]
    command: Cli,
}

#[derive(Subcommand)]
pub enum Cli {
    #[command(name = "g", long_about, about)]
    Generate(GenerateCommand),
}

// Generate Shim
#[derive(Parser)]
pub struct GenerateCommand {
    #[command(subcommand)]
    pub subcommand: GenerateSubcommand,
}

#[derive(Subcommand)]
pub enum GenerateSubcommand {
    #[command(long_about, about)]
    Feature {
        #[arg(value_name = "NAME")]
        name: String,

        #[arg(short = 'c', value_enum)]
        context: GeneratorContext,
    },

    #[command(long_about, about)]
    Component {
        #[arg(value_name = "FEATURE")]
        feature_name: String,

        #[arg(value_name = "COMPONENT")]
        component_name: String,

        #[arg(short = 'c', value_enum)]
        context: GeneratorContext,
    },

    #[command(long_about, about)]
    Service {
        #[arg(value_name = "FEATURE")]
        feature_name: String,

        #[arg(value_name = "SERVICE")]
        service_name: String,
    },

    #[command(long_about, about)]
    Controller {
        #[arg(value_name = "FEATURE")]
        feature_name: String,

        #[arg(value_name = "CONTROLLER")]
        controller_name: String,
    },
}

impl Cli {
    pub fn parse_args() -> Self {
        let args = Args::parse();
        args.command
    }
}

#[macro_export]
macro_rules! shim {
    (
        $command:ident -> $variant:ident {
            $($pattern:tt)*
        }
    ) => {
        paste::paste! {
            [<$command Command>] {
                subcommand: [<$command Subcommand>]::$variant {
                    $($pattern)*
                },
            }
        }
    };
}
