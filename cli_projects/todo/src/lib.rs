use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

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

pub fn delete_task(index: usize) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    let mut tasks = String::new();
    let _ = file.read_to_string(&mut tasks);
    let result_task: Vec<&str> = tasks
        .lines()
        .enumerate()
        .into_iter()
        .filter(|&(i, _)| i != index)
        .map(|(_, e)| e)
        .collect();

    let _ = file.set_len(0);
    for task in result_task {
        let _ = file.write(format!("{:}\n", task).as_bytes());
    }
}

pub fn complete_task() {}

pub fn edit_task() {}

pub fn list_task() {}
