use std::sync::Mutex;

#[tokio::main]
async fn main() {
    let data = Mutex::new(10);
    // Lifetime of guard starts here.
    let guard = data.lock().expect("Can't acquire lock");
    test().await;
    // Ends after this.
}

async fn test() {}
