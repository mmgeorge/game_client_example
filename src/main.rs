
#[macro_use] extern crate serde_json;
extern crate hyper;

use hyper::Client;
use hyper::status::StatusCode;
use serde_json::{Value, from_reader};

fn main() {
    let client = Client::new();

    // Assuming we are hosting the server locally:
    let url = "http://localhost:8080/api/connect_four.svc/Games(0)";
    let response = client.get(url).send().unwrap();
    assert_eq!(response.status, StatusCode::Ok); // sanity check

    // Parse JSON
    let value: Value = from_reader(response).expect("Unable to parse response!");
    println!("Body: {:?}", value);
    println!("Got Board: id = {}, height = {}, width = {}",
             value["id"], value["height"], value["width"]);
}



