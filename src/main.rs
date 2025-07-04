mod todo;

use todo::Todo;

// Test 1

fn main() {
    let mut my_todo = Todo::new("Learn Rust");
    my_todo.set_description("Study Rust moduled and structs");

}
