use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    a: u64,
    b: u64,
}

fn main() {
    let rust_object = Foo { a: 100, b: 200 };

    // Converting from rust type to data format type. Json in this case.
    let data_format_object = serde_json::to_string(&rust_object).unwrap();
    println!("Data format type, Json- {:?}", data_format_object);

    let rust_object: Foo = serde_json::from_str(&data_format_object).unwrap();
    println!("Rust type= {:?}", rust_object);
}
