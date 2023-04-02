use clap::{Parser, CommandFactory, Command};

#[derive(Debug, Parser)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
    #[arg(short = 's', long, default_value = "0.0.0.0")]
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
