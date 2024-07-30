use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PostRegister {
    pub uuid: String,
    pub publickey: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostGet {
    pub uuid: String,
    pub gettime: String,
    pub gettimesignature: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostSend {
    pub sender: String,
    pub receiver: String,
    pub content: String,
    pub sendtime: String,
    pub sendtimesignature: String,
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
