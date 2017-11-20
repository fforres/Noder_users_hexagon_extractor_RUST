extern crate reqwest;
use reqwest::Client;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let response = make_request();
    match response.status() {
        StatusCode::Ok => handle_response(response),
        _ => panic!("Error!"),
    }
}


fn make_request() -> reqwest::Response {
    let api_token = "a_key_xoxp-4140681291-4140681329-129010607461-3734037a8962a7554f94875ab336d69a";
    let client = Client::new();
    let mut map = HashMap::new();

    map.insert("token", api_token);

    let response = client
        .post("https://slack.com/api/users.list")
        .form(&map)
        .send()
        .expect("Failed to send");

    response
}

fn handle_response(mut response: reqwest::Response) {
    let content = response.text();
    let file = "./lorem_ipsum.txt";
    match response.text() {
        Err(why) => {
            panic!("Error! {}",
            why.description())
        },
        String(param) => create_file(&content, file, param)
    }
}

fn create_file(content: &reqwest::Result<String>, path: &str) {
    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}",
            display,
            why.description())
        },
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
    Err(why) => {
        panic!("couldn't write to {}: {}",
        display,
        why.description())
    },
    Ok(_) => println!("successfully wrote to {}", display),
}
    // buffer.write(&content);
}