use actix_web::{get, web, App, HttpServer, HttpResponse};
use serde::{Serialize, Deserialize};
use quick_xml::se::to_string;

// Define a simple struct
#[derive(Debug, Serialize, Deserialize)]
struct MyStruct {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Wrapper {
    result: Vec<MyStruct>,
}

// Define a handler to return XML
#[get("/data")]
async fn get_data() -> HttpResponse {
    // Create some sample data
    let data: Vec<MyStruct> = vec![
        MyStruct { id: 1, name: "Item 1".to_string() },
        MyStruct { id: 2, name: "Item 2".to_string() },
        MyStruct { id: 3, name: "Item 3".to_string() },
    ];

    let data = Wrapper {
        result: data,
    };

    // Convert data to XML
    match to_string(&data) {
        Ok(xml_data) => HttpResponse::Ok()
            .content_type("application/xml")
            .body(xml_data),
        Err(err) => {
            eprintln!("Error serializing data to XML: {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
