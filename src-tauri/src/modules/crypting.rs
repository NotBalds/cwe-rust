use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
pub use base64::prelude::BASE64_STANDARD as b64;

pub fn xor(text: &str, key: &str) -> String {
    let mut encrypted_text = String::new();
    
    for (i, c) in text.chars().enumerate() {
        encrypted_text.push(char::from(c as u8 ^ key.chars().nth(i % key.len()).unwrap_or(' ') as u8));
    }
    
    encrypted_text
}


