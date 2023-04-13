use clap::{Parser, CommandFactory, Command};

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    /// Server port
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    /// Server host
    #[arg(short = 's', long, default_value = "127.0.0.1")]
    pub host: String
}

impl Default for Cli {
    fn default() -> Self {
        Self::parse()
    }
}

impl Cli {
    pub fn cmds() -> Command {
        Self::command()
    }
}
