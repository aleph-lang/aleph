extern crate argparse;

use argparse::{ArgumentParser, Store};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct AlephResult {
    response: String,
}

#[actix_web::main]
async fn main() {
    let mut content_type = "json".to_string();
    let mut return_type = "ale".to_string();
    let mut transformer_param = "".to_string();
    {  
        let mut ap = ArgumentParser::new();
        ap.set_description("Multi compiler");
        ap.refer(&mut content_type) .add_option(&["--in_type", "-i"], Store, "Input type");
        ap.refer(&mut transformer_param) .add_option(&["--transformer_list", "-t"], Store, "Transformer list");
        ap.refer(&mut return_type) .add_option(&["--out_type", "-o"], Store, "Output type");
        ap.parse_args_or_exit();
    }

    let transformer_list : Vec<&str> = transformer_param.split(",").collect();
 
    let mut content = String::new();
    io::stdin().read_to_string(&mut content).unwrap();

    let request = serde_json::json!({
        "content_type" : content_type,
        "content" : content,
        "return_type" : return_type,
        "transformer_list" : transformer_list
    });

    let client = awc::Client::default();
    let response = client.post("http://localhost:8080/").send_json(&request).await;
    let data = response.unwrap().json::<AlephResult>().await;

    println!("{}", data.unwrap().response);
}
