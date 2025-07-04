use serde::{Deserialize, Serialize}; // Make sure to add the derive feature in Cargo.toml

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub title: String,
    pub description: Option<String>,
    completed: bool
}


impl Todo {
    // Constructor
    pub fn new(title: &str) -> Self {
        Todo {
            title: title.to_string(),
            description: None,
            completed: false
        }
    }

    // Methods
    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }
}