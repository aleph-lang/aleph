use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde_json::json;
use serde::{Deserialize, Serialize};
use std::fs;

mod filter;
mod syntax;

use crate::filter::parser::parse;
use crate::filter::gen::generate;
use crate::filter::transform_dispatcher;

#[derive(Debug, Serialize, Deserialize)]
struct AlephEntry {
    content_type: String,
    content: String,
    return_type: String,
    transformer_list: Option<Vec<String>>
}


/// This handler uses json extractor
async fn index(item: web::Json<AlephEntry>) -> HttpResponse {
    let parsed_content: syntax::AlephTree = parse(item.0.content_type, item.0.content);

    let transformed_content: syntax::AlephTree = match item.0.transformer_list {
        Some(list)=> transform_dispatcher(list, parsed_content),
        None => parsed_content
    };
    
    let output = generate(item.0.return_type, transformed_content);

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
