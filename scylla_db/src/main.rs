use actix_web::{get, post, web, App, Handler, HttpResponse, HttpServer, Responder};

use lazy_static::lazy_static;
use scylla::{transport::session, Session, SessionBuilder};
use tokio::runtime::Handle;

lazy_static! {
    pub static ref DB_SESSION: Session = {
        let handle = Handle::current();
        handle.block_on(async {
            SessionBuilder::new()
            .build()
            .await
            .expect("Issue building session")
        })
    };
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let session = &DB_SESSION;
    HttpResponse::Ok().body(req_body)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let session = &DB_SESSION;
    HttpServer::new(|| {
        App::new()
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}