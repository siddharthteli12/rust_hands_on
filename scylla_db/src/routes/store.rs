use crate::{actix_state::WebState, insert_to_db};
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::sync::Arc;

#[post("/db_store")]
async fn db_store(
    id: HttpRequest,
    req_body: String,
    state: web::Data<Arc<WebState>>,
) -> impl Responder {
    let id = id.headers().get("ID").unwrap().to_str().unwrap();
    insert_to_db(&state.session, id.to_string(), req_body).await;
    HttpResponse::Ok().body("Succesful")
}
