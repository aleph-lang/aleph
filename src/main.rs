use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::fs;

mod filter;

use crate::filter::generate;

#[derive(Debug, Serialize, Deserialize)]
struct AlephEntry {
    content_type: String,
    content: String,
    return_type: String,
    transformer_list: Option<Vec<String>>
}

/// This handler uses json extractor
async fn index(item: web::Json<AlephEntry>) -> HttpResponse {
    let output = generate(item.0.content_type, item.0.content, item.0.transformer_list, item.0.return_type);

    let res = json!({"response" : output});

    HttpResponse::Ok().json(res)
}

async fn infos() -> HttpResponse {
    let contents = fs::read_to_string("conf/default.conf")
        .expect("Should have been able to read the file");
    let res = json!({"response" : contents});

    HttpResponse::Ok().json(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default())
            .route("/", web::post().to(index))
            .route("/infos", web::get().to(infos))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
