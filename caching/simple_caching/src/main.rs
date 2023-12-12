use std::{cell::Cell, collections::HashMap, io, ops::Deref};

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
#[derive(Clone, Copy)]
struct Response {
    value: u128,
}

impl AppState {
    fn get_response(&self, id: u32) -> io::Result<Response> {
        // Return from cache or else from db.
        if let Some(value) = self.cache.take().get(&id) {
            return Ok(*value);
        }
        let value = self.db.process_request_id(id);
        let mut map = self.cache.take();
        map.insert(id, *value.as_ref().unwrap());
        self.cache.set(map);
        value
    }
}

fn main() {}
