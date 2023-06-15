use std::env;
use std::process::exit;
use std::{
    ffi::{OsStr, OsString},
    fs,
};
fn main() {
    let args: Vec<String> = env::args().collect();
    let result = search_dir(&args[1]);
    println!("{:?}", result.join(OsStr::new(" ")));
}

// Returns entries & folders on given path.
pub fn search_dir(path: &str) -> Vec<OsString> {
    // Path directory.
    let dir = match fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Path doesn't exist");
            exit(-1);
        }
    };
    // To store entries & folders.
    let mut entries = vec![];
    for file in dir {
        entries.push(file.unwrap().file_name());
    }
    entries
}
