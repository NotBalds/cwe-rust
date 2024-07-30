use {
    openssl::{
        rsa::Rsa, 
        symm::Cipher
    },
    base64::{
            prelude::BASE64_STANDARD as b64, 
            Engine
        }
};

const BITS: u32 = 2048;

pub fn xor(text: String, key: String) -> String {
    let mut encrypted_text = String::new();
    
    for (i, c) in text.chars().enumerate() {
        encrypted_text.push(char::from(c as u8 ^ key.chars().nth(i % key.len()).unwrap_or(' ') as u8));
    }
    
    encrypted_text
}

pub fn gen_keys(passphrase: String) -> (String, String) {
    let rsa = Rsa::generate(BITS).unwrap();
    
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_256_cbc(), passphrase.as_bytes())
                                    .expect("Failed to convert private key to pem and encrypt");
    let public_key: Vec<u8> = rsa.public_key_to_pem_pkcs1()
                                    .expect("Failed to convert public key to pem");
    let private_key: String = String::from_utf8(private_key)
                                    .expect("Can't convert private key from Vec<u8> to String");
    let public_key: String = String::from_utf8(public_key)
                                    .expect("Can't convert public key from Vec<u8> to String");

    (private_key, public_key)
}

pub fn public_from_pem_to_base64(public_key_pem: String) -> String {
    let public_key = Rsa::public_key_from_pem_pkcs1(public_key_pem.as_bytes()).expect("Can't convert public key from pem to object");
    b64.encode(public_key.public_key_to_der().unwrap())
}
