// Simple custom macro impl.
macro_rules! println_wrapper {
    ($expression:expr) => {
        println!("{:?}", $expression);
    };
}

fn main() {
    println_wrapper!("Hello, world!");
}
