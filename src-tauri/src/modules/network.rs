use reqwest::blocking::Client;
use std::collections::HashMap;

pub fn post(url: &str, body: HashMap<&str, &str>) {
    let client = Client::new();
    let resp = match client.post(url).json(&body).send() {
        Ok(resp) => resp.text().unwrap(),
        Err(err) => panic!("Error: {}", err)
    };
    println!("200 {} end", resp)
}
