use std::{fs::OpenOptions, io::Write};

pub fn add_task(task: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    if let Err(e) = writeln!(file, "{:}", task) {
        eprintln!("Issue writing to file due to {:}", e);
    }
}

pub fn delete_task() {}

pub fn complete_task() {}

pub fn edit_task() {}

pub fn list_task() {}
