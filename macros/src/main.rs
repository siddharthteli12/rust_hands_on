// Eg 1 custom vec.
macro_rules! custom_vec {
    ($($e:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($e);)*
        v
    }};
}

// Eg 2 custom print.
macro_rules! println_wrapper {
    ($exp1:expr, $exp2:expr) => {
        println!("{:?}", $exp2);
    };
}

fn main() {
    let vec1 = custom_vec![1, 2, 3];
    println_wrapper!("{:?}", vec1);
}
