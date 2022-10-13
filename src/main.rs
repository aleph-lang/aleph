use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::Write;
use std::process::{Command, Stdio};

pub mod syntax;

#[derive(Debug, Serialize, Deserialize)]
struct AlephEntry {
    content_type: String,
    content: String,
    return_type: String
}


/// This handler uses json extractor
async fn index(item: web::Json<AlephEntry>) -> HttpResponse {
    println!("model: {:?}", &item);
    let content = item.0.content.clone();


    //example for parsing json using syntax enum
    let parsed_content: syntax::AlephTree = serde_json::from_str(&item.0.content).unwrap();
    println!("parsed_content: {:?}", &parsed_content);

    
    // run json2ale
    let child = Command::new("./alephc")
                         .current_dir("../aleph-contrib")
                         .arg("-conf")
                         .arg("conf/json2ale.conf")
                         .arg("-stdout").arg("true")
                         .stdin(Stdio::piped())
                         .stdout(Stdio::piped())
                         .spawn().unwrap();
    child.stdin.as_ref().unwrap().write(content.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();

    let res = json!({"response" : String::from_utf8_lossy(&output.stdout)});

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
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .service(web::resource("/").route(web::post().to(index)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
