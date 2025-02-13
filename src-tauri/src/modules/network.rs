use crate::base;
use crate::modules::config;
use crate::modules::json;
use reqwest::blocking::Client;
use std::collections::HashMap;

fn post(
    url: String,
    body: HashMap<String, String>,
) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let client = Client::new();
    client.post(url).json(&body).send()
}

pub fn register(uuid: String, publickey: String) -> u16 {
    let data = json::PostRegister { uuid, publickey };
    let body = json::to_hashmap(&data);
    match post(config::url("register"), body) {
        Ok(response) => response.status().as_u16(),
        Err(e) => {
            base::log(
                &format!("Error while sending POST request to /register: {}", e),
                1,
            );
            0
        }
    }
}

pub fn get(uuid: String, gettime: String, gettimesignature: String) -> (u16, json::GetResponse) {
    let data = json::PostGet {
        uuid,
        gettime,
        gettimesignature,
    };
    let body = json::to_hashmap(&data);
    match post(config::url("get"), body) {
        Ok(response) => (
            response.status().as_u16(),
            match response.json::<json::GetResponse>() {
                Ok(data) => data,
                Err(err) => {
                    base::log(&format!("Error while parsing JSON response: {}", err), 1);
                    Vec::from([])
                }
            },
        ),
        Err(e) => {
            base::log(
                &format!("Error while sending POST request to /get: {}", e),
                1,
            );
            (0, Vec::from([]))
        }
    }
}

pub fn send(
    sender: String,
    receiver: String,
    content: String,
    sendtime: String,
    sendtimesignature: String,
) -> u16 {
    let data = json::PostSend {
        receiver,
        sender,
        content,
        sendtime,
        sendtimesignature,
    };
    let body = json::to_hashmap(&data);

    match post(config::url("send"), body) {
        Ok(response) => response.status().as_u16(),
        Err(e) => {
            base::log(
                &format!("Error while sending POST request to /send: {}", e),
                1,
            );
            0
        }
    }
}
