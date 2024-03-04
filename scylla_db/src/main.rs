use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};

mod state;
use state::APP_STATE;
mod actix_state;
use actix_state::WebState;
use std::sync::Arc;
mod db;
use db::*;

#[post("/store")]
async fn echo(req_body: String, state: web::Data<Arc<WebState>>) -> impl Responder {
    let value; {
        let mut counter = state.counter.lock().unwrap();
        *counter += 1;
        value = *counter;
    }
    insert_to_db(&state.session, value).await;
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = WebState::web_build().await;
    init_db(&state.session).await;
    let state = Arc::new(state);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
