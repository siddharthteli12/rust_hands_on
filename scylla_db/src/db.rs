use scylla::Session;

pub async fn init_db(session: &Session) {
    let query = "CREATE TABLE IF NOT EXISTS scylla.test (key BIGINT PRIMARY KEY, value BIGINT)";
    session.query(query, ()).await.unwrap();
}

pub async fn insert_to_db(session: &Session, counter: i64) {
    let query = "INSERT INTO scylla.test (key, value) VALUES (?, ?)";
    session.query(query, (&counter, &counter)).await.unwrap();
}
