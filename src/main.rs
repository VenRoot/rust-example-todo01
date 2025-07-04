mod todo;
mod storage;

use anyhow::{Context, Result};
use todo::Todo;
use storage::Storage;

// Test 1

fn main() -> Result<()> {

    // Example usage of the Todo app
    let storage = Storage::new("my_todos");


    // Create some todos
    let mut todos = vec![
        Todo::new("Learn Rust"),
        Todo::new("Build a Todo App")
    ];

    todos[0].set_description("Focus on modules and ownership");
    todos[0].complete();
    
    todos[1].set_description("Use serde for serialisation");

    // Save todos to file
    storage.save_todos(&todos)
        .context("Failed to save todos")?;

    println!("Todos saved successfully");

    // Load todos from file
    let loaded_todos = storage.load_todos()
        .context("Failed to load todos")?;


    println!("Loaded {} todos:", loaded_todos.len());
    for (i, todo) in loaded_todos.iter().enumerate() {
        println!(
            "{}. {} [{}]{}",
            i + 1,
            todo.title,
            if todo.is_completed() { "✅" } else { " " },
            todo.description.as_ref().map_or(String::new(), |d| format!("*: {}", d))
        );
    }

    Ok(())

}
