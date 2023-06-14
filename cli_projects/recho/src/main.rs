use std::env;

fn main() {
    // Read cli args.
    let mut cli_args: Vec<String> = env::args().collect();
    // Store user input with whitespace.
    let mut input_string = String::new();
    // Remove first index which is name of program.
    cli_args.remove(0);
    // Iterate over args & prefix with whitespace.
    for string in cli_args {
        input_string.push_str(&format!(" {:}", string));
    }
    println!("{:}", input_string.trim_start());
}
