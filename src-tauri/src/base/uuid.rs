use {
    crate::{
        modules::network,
        base::filesystem
    },
    uuid::Uuid
};

fn generate() -> String {
    let uuid = Uuid::new_v4().to_string();
    uuid
}

pub fn register(public_key: String) -> String {
    let mut uuid = generate();
    while !(network::register(uuid.clone(), public_key.clone()) == 200) {
        uuid = generate();
    };
    uuid
}

pub fn get() -> String {
    filesystem::cat(&filesystem::new_path("uuid")).unwrap()
}
