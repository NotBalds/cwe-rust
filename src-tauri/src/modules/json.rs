use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PostRegister {
    uuid: String,
    publickey: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostGet {
    uuid: String,
    gettime: String,
    gettimesignature: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostSend {
    sender: String,
    receiver: String,
    content: String,
    sendtime: String,
    sendtimesignature: String,
}

pub fn to_hashmap<Type: Serialize>(obj: &Type) -> HashMap<String, String> {
    let json_value = serde_json::to_value(obj).unwrap();
    json_value.as_object()
        .unwrap()
        .iter()
        .filter_map(|(key, value)| {
            if let Some(value_str) = value.as_str() {
                Some((key.clone(), value_str.to_string()))
            } else {
                None
            }
        })
        .collect()
}
