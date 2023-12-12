use std::{collections::HashMap, io};

// DbPool connection
struct DbPool;

impl DbPool {
    pub fn process_request_id(&self, id: u32) -> io::Result<Response> {
        // Very expensive db operation.
        Ok(Response {
            value: "Hello".to_string(),
        })
    }
}
struct AppState {
    db: DbPool,
    cache: HashMap<u32, Response>,
}

// Response
#[derive(Clone)]
struct Response {
    value: String,
}

impl AppState {
    fn get_response(&self, id: u32) -> io::Result<Response> {
        // Return from cache or else from db.
        if let Some(value) = self.cache.get(&id) {
            return Ok(value.clone());
        } else {
            self.db.process_request_id(id)
        }
    }
}

fn main() {}
