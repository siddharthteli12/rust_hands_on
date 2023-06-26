use std::{
    fs::OpenOptions,
    io::{Read, Write},
};

use colored::Colorize;

pub fn add_task(task: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();

    writeln!(file, "{:}", task).expect("Issue writing to file");
}

pub fn delete_task(index: usize) {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open("todo.txt")
        .unwrap();

    let mut tasks = String::new();
    file.read_to_string(&mut tasks)
        .expect("Issue reading to string");

    let result_task: String = tasks
        .lines()
        .enumerate()
        .into_iter()
        .filter(|&(i, _)| i != index)
        .map(|(_, e)| format!("{:}\n", e))
        .collect();

    file.set_len(0).expect("Issue in settin len to zero");
    file.write(result_task.as_bytes())
        .expect("Issue writing to file");
}

pub fn complete_task(index: usize) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .open("todo.txt")
        .unwrap();

    let mut result_tasks = String::new();
    file.read_to_string(&mut result_tasks)
        .expect("Issue reading from file");
    let mut tasks: Vec<&str> = result_tasks.lines().collect();

    if tasks[index].starts_with("*") {
        return;
    } else {
        let completed_task = format!("*{:}", tasks[index]);
        tasks[index] = &completed_task;
        file.set_len(0).expect("Issue setting file len to 0");
        for task in tasks {
            file.write(format!("{:}\n", task).as_bytes())
                .expect("Issue writing to file");
        }
    }
}

pub fn edit_task(index: usize, new_task: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .append(true)
        .create(true)
        .open("todo.txt")
        .unwrap();
    let mut tasks = String::new();
    file.read_to_string(&mut tasks)
        .expect("Issue reading to string");

    let mut tasks: Vec<&str> = tasks.lines().collect();

    tasks[index] = &new_task;
    file.set_len(0).expect("Issue setting file len to 0");
    for task in tasks {
        file.write(format!("{:}\n", task).as_bytes())
            .expect("Issue writing to file");
    }
}

pub fn list_task() {
    let mut file = OpenOptions::new()
        .read(true)
        .append(true)
        .open("todo.txt")
        .unwrap();

    let mut result_tasks = String::new();
    file.read_to_string(&mut result_tasks)
        .expect("Issue reading to string");

    for (index, task) in result_tasks.lines().enumerate() {
        if task.chars().nth(0).unwrap() == '*' {
            println!(
                "{:} {:}",
                index.to_string().bold(),
                task[1..].strikethrough()
            );
        } else {
            println!("{:} {:}", index.to_string().bold(), task.green());
        }
    }
}
