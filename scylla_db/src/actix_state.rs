use crate::APP_STATE;

use scylla::{Session, SessionBuilder};

use std::sync::Mutex;
pub struct WebState {
    pub session: Session,
}

impl WebState {
    pub async fn web_build() -> Self {
        let db_url = format!("{}:{}", &APP_STATE.db_host, &APP_STATE.db_port);
        println!("Strating db on {:?}", db_url);
        let session: Session = SessionBuilder::new()
            .known_node(db_url)
            .build()
            .await
            .unwrap();
        Self { session }
    }
}
