use lazy_static::lazy_static;

lazy_static! {
    static ref URL: String = "https://google.com".to_string();
}
fn main() {
    println!("Url - {:?}", *URL);
}
