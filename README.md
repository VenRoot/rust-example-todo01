# ven-todo-rs

A simple and efficient command-line todo application written in Rust with YAML storage.

## Features

- Create and manage todo items with titles and optional descriptions
- Mark todos as complete
- List all your todos with completion status
- Cross-platform support (Linux, macOS, Windows)
- Data stored in YAML format for human readability
- Follows XDG Base Directory specification on Linux

## Installation

### From crates.io

```bash
cargo install ven-todo
```
From AUR (Arch Linux)
```bash
yay -S ven-todo-rs
# or
paru -S ven-todo-rs
```

From Source
```bash
git clone https://github.com/VenRoot/rust-example-todo01.git
cd rust-example-todo01
cargo build --release
```
## Usage 
Adding a new todo
```bash
ven-todo add "Learn Rust" --description "Focus on ownership and borrowing"
```
Listing your todos
```bash
ven-todo list
```
Output example:
```
1. Learn Rust [ ] - Focus on ownership and borrowing
2. Build CLI app [✓]
```
Marking a todo as complete
```bash
ven-todo complete 1
```
## Storage
Todos are stored in a YAML file in the following location:

* Linux: `$XDG_DATA_HOME/my_todos.yaml` or `~/.local/share/my_todos.yaml`
* Windows: TBD
* macOS: TBD

## Development
Prerequisites
* Rust 1.7x or higher
* Cargo

Running tests
```bash
cargo test
```
## License
This project is licensed under the GLP 3.0 License - see the LICSENSE file for details

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request
