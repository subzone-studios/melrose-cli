use clap::ValueEnum;

#[derive(ValueEnum, Clone)]
pub enum GeneratorContext {
    Client,
    Server,
    Shared,
}

impl AsRef<str> for GeneratorContext {
    fn as_ref(&self) -> &str {
        match self {
            Self::Client => "client",
            Self::Server => "server",
            Self::Shared => "shared",
        }
    }
}
