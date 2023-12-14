use std::{cell::Cell, collections::HashMap, io};

// DbPool connection
struct DbPool;

impl DbPool {
    pub fn process_request_id(&self, id: u32) -> io::Result<Response> {
        // Very expensive db operation.
        Ok(Response { value: 10000000 })
    }
}
struct AppState {
    db: DbPool,
    cache: Cell<HashMap<u32, Response>>,
}

// Response
#[derive(Clone, Copy, Debug)]
struct Response {
    value: u128,
}

impl AppState {
    fn get_response(&self, id: u32) -> io::Result<Response> {
        let mut cache = self.cache.take();
        let result;

        if let Some(&value) = cache.get(&id) {
            result = value;
        } else {
            result = self.db.process_request_id(id).unwrap();
            cache.insert(id, result);
        }
        self.cache.set(cache);
        Ok(result)
    }
    fn new() -> Self {
        Self {
            db: DbPool,
            cache: Cell::new(HashMap::default()),
        }
    }
}

fn main() {
    let state = AppState::new();
    state.get_response(10).unwrap();
    state.get_response(11).unwrap();
}
