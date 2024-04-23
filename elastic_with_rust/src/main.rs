use actix_web::{
    web::{self},
    App, HttpServer, Responder,
};
use elasticsearch::{Elasticsearch, IndexParts};
use serde_json::json;

async fn register(client: web::Data<Elasticsearch>) -> impl Responder {
    println!("User registration api called");
    let index_name = "user_registration";
    let id = format!("{}-{}", index_name, uuid::Uuid::new_v4());
    let response = client
        .index(IndexParts::IndexId(index_name, id.as_str()))
        .body(json!({
            "id": 1,
            "user": "siddharth",
            "post_date": "2024-04-23T00:00:00Z",
            "message": "User kimchy created"
        }))
        .send()
        .await
        .unwrap();
    println!("Response from elastic clinet {:?}", response.status_code());
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let elastic_client = web::Data::new(Elasticsearch::default());
    HttpServer::new(move || {
        App::new()
            .app_data(elastic_client.clone())
            .route("/register", web::get().to(register))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
