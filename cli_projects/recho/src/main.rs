use std::env;

fn main() {
    // Read cli args.
    let mut cli_args: Vec<String> = env::args().collect();
    // Remove first index, which is name of program.
    cli_args.remove(0);
    // Join string list with whitespace.
    println!("{:}", cli_args.join(" "));
}
