use anyhow::{Context, Result};
use serde_yaml;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::env;

use crate::todo::Todo;


pub struct Storage {
    file_path: PathBuf
}

impl Storage {
    pub fn new(file_name: &str) -> Self {
        let mut base_dir = Self::data_dir();
        base_dir.push("todo-rs");

        fs::create_dir_all(&base_dir).ok(); // Create data dir if it doesn't exist

        let mut path = base_dir.join(file_name);

        if !path.to_str().unwrap_or_default().ends_with(".yaml") {
            path.set_extension("yaml");
        }

        Storage { file_path: path } // Return this
    }

    fn data_dir() -> PathBuf {
        if let Some(dir) = env::var_os("XDG_DATA_HOME") {
            PathBuf::from(dir)
        } else if let Some(home) = env::var_os("HOME").or_else(|| env::var_os("USERPROFILE")) {
            PathBuf::from(home).join(".local").join("share")
        } else {
            PathBuf::from(".")
        }
    }

    pub fn save_todos(&self, todos: &[Todo]) -> Result<()> {
        let yaml = serde_yaml::to_string(&todos)
            .context("Failed to serialize todos")?;

        let mut file = File::create(&self.file_path)
            .context("Failed to create todo file")?;

        file.write_all(yaml.as_bytes())
            .context("Failed to write to todo file")?;

        Ok(())
    }

    pub fn load_todos(&self) -> Result<Vec<Todo>> {
        if !self.file_path.exists() {
            return Ok(Vec::new())
        }

        let mut file = File::open(&self.file_path)
            .context("Failed to open todo file")?; // Note the ? here. This resolves the Result from the File, so we have the `File` type 

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read todo file")?;

        let todos: Vec<Todo> = serde_yaml::from_str(&contents)
            .context("Failed to parse todos from file")?;

        Ok(todos)
    }
}