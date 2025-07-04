use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo", version, about = "Simple CLI todo list written in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    List,
    Complete {
        index: usize
    }
}