extern crate argparse;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::io;
use std::io::Read;
use std::fs;

mod filter;
use crate::filter::generate;

#[actix_web::main]
async fn main() {
    let mut content_type = "ale".to_string();
    let mut return_type = "ale".to_string();
    let mut transformer_param = "".to_string();
    let mut infos = false;
    {  
        let mut ap = ArgumentParser::new();
        ap.set_description("Multi compiler");
        ap.refer(&mut content_type).add_option(&["--in_type", "-i"], Store, "Input type");
        ap.refer(&mut transformer_param).add_option(&["--transformer_list", "-t"], Store, "Transformer list");
        ap.refer(&mut return_type).add_option(&["--out_type", "-o"], Store, "Output type");
        ap.refer(&mut infos).add_option(&["-g", "--infos"], StoreTrue, "Get informations");
        ap.parse_args_or_exit();
    }

    let response = if !infos {
        let transformer_list : Vec<String> = transformer_param.split(",").map(|s| s.to_string()).collect();
 
        let mut content = String::new();
        io::stdin().read_to_string(&mut content).unwrap();
     
        generate(content_type, content, Some(transformer_list), return_type)
    } else {
        let contents = fs::read_to_string("conf/default.conf")
        .expect("Should have been able to read the file");
        contents
    };
    
    println!("{}", response);
}
