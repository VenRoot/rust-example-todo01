pub struct Todo {
    title: String,
    description: Option<String>,
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