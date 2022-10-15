use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde_json::json;
use serde::{Deserialize, Serialize};

mod filter;
mod syntax;

use crate::filter::gen::Gen;
use crate::filter::gen::ale::AleGen as ale_gen;


#[derive(Debug, Serialize, Deserialize)]
struct AlephEntry {
    content_type: String,
    content: String,
    return_type: String
}


/// This handler uses json extractor
async fn index(item: web::Json<AlephEntry>) -> HttpResponse {
    //example for parsing json using syntax enum
    let parsed_content: syntax::AlephTree = serde_json::from_str(&item.0.content).unwrap();
    
    // run json2ale
    let output = ale_gen::generate(parsed_content);

    let res = json!({"response" : output});

    HttpResponse::Ok().json(res) // <- send response
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .app_data(web::JsonConfig::default().limit(19384)) // <- limit size of the payload (global configuration)
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
