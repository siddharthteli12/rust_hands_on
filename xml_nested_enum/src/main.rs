use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Bread {
    sauce: Sauce,
}

#[derive(Deserialize)]
enum Sauce {
    Garlic,
    Onion,
}

#[get("/")]
async fn hello(data: String) -> impl Responder {
    match quick_xml::de::from_str::<Bread>(&data) {
        Ok(_) => HttpResponse::Ok().body("Hello world!"),
        Err(e) => HttpResponse::Ok().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
