mod todo;
mod storage;
mod cli;

use anyhow::{Context, Result};
use todo::Todo;
use storage::Storage;
use clap::Parser;
use cli::{Cli, Commands};

// Test 1

fn main() -> Result<()> {

    let cli = Cli::parse();
    let storage = Storage::new("my_todos");

    let mut todos = storage.load_todos().unwrap_or_default();

    match cli.command {
        Commands::Add { title, description } => {
            let mut todo = Todo::new(&title);
            if let Some(desc) = description.as_deref() {
                todo.set_description(desc);
            }
            todos.push(todo);
            storage
                .save_todos(&todos)
                .context("Failed to save todos")?;
            println!("Todo added");
        }
        Commands::List => {
            for (i, todo) in todos.iter().enumerate() {
                println!(
                    "{}. {} [{}]{}",
                    i + 1,
                    todo.title,
                    if todo.is_completed() { "x" } else { " " },
                    todo
                        .description
                        .as_ref()
                        .map(|d| format!(" - {}", d))
                        .unwrap_or_default()
                );
            }
        }
        Commands::Complete { index } => {
            if let Some(todo) = todos.get_mut(index - 1) {
                todo.complete();
                storage
                    .save_todos(&todos)
                    .context("Failed to save todos")?;
                println!("Todo {} marked complete", index);
            } else {
                println!("No todo found with index {}", index);
            }
        }
    }

    Ok(())

}
