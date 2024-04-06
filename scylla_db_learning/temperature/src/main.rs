mod db;
mod result;
mod temperature_measurement;

use db::create_session;
use result::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Connecting to db");
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    let session = create_session(&uri).await?;
    Ok(())
}
