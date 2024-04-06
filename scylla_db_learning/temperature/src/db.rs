use crate::Result;
use scylla::{Session, SessionBuilder};

pub async fn create_session(url: &str) -> Result<Session> {
    SessionBuilder::new()
        .known_node(url)
        .build()
        .await
        .map_err(From::from)
}
