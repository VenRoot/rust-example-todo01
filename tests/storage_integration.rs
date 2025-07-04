use todo01::{storage::Storage, todo::Todo};
use tempfile::tempdir;
use std::env;

#[test]
fn save_and_load_roundtrip() {
    let dir = tempdir().expect("create temp dir");
    // ! Usually, this should not be done, but it's safe as we use single-threating
    unsafe {
        // set XDG_DATA_HOME so Storage writes into our temp directory
        env::set_var("XDG_DATA_HOME", dir.path());
    }

    let storage = Storage::new("test_todos");

    let mut todo1 = Todo::new("learn rust");
    todo1.set_description("practice borrowing");

    let mut todo2 = Todo::new("write tests");
    todo2.set_description("practice borrowing");

    let todos = vec![todo1.clone(), todo2.clone()];
    storage.save_todos(&todos).expect("save todos");

    let loaded = storage.load_todos().expect("load todos");

    assert_eq!(todos, loaded);

    // ! Again, single threating
    unsafe {
        env::remove_var("XDG_DATA_HOME");
    }
}