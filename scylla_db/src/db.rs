use crate::types::*;
use scylla::IntoTypedRows;
use scylla::Session;
pub async fn init_db(session: &Session) {
    let query = "CREATE TABLE IF NOT EXISTS user.info (key TEXT PRIMARY KEY, value TEXT)";
    session.query(query, ()).await.unwrap();
}

pub async fn insert_to_db(session: &Session, id: String, payload: String) {
    let query = "INSERT INTO user.info (key, value) VALUES (?, ?)";
    session.query(query, (id, &payload)).await.unwrap();
}

pub async fn get_from_db(session: &Session, id: String) {
    let query = "SELECT * FROM user.info WHERE key = ?";
    let rows = session
        .query(query, vec![id])
        .await
        .expect("Failed to execute query")
        .rows
        .unwrap()
        .into_typed::<Payload>()
        .collect::<Result<Vec<Payload>, _>>()
        .unwrap();
}
