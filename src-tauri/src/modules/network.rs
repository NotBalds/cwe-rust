use reqwest::blocking::Client;
use std::collections::HashMap;
use crate::modules::json;
use crate::modules::config;

fn post(url: String, body: HashMap<String, String>) -> u16 {
   let client = Client::new();
    let resp = match client.post(url).json(&body).send() {
        Ok(resp) => resp.status().as_u16(),
        Err(err) => panic!("Error: {}", err)
    };
    resp
}

pub fn register(uuid: String, publickey: String) -> u16 {
    let data = json::PostRegister {
        uuid,
        publickey,
    };
    let body = json::to_hashmap(&data);
    post(config::url("register"), body)
}

pub fn get(uuid: String, gettime: String, gettimesignature: String) -> u16 {
    let data = json::PostGet {
        uuid,
        gettime,
        gettimesignature,
    };
    let body = json::to_hashmap(&data);
    post(config::url("get"), body)
}


