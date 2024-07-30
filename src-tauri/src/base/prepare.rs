use {
    std::io,
    crate::{
        base::{
            filesystem,
            uuid
        },
        modules::crypting
    }
};

pub fn run(passphrase: String) -> io::Result<()> {
    filesystem::mkAllDirs(
        &filesystem::new_path("base-keys")
    );
    
    // Generating keys
    let my_keys = crypting::gen_keys(passphrase.clone());
    let my_private_key = my_keys.0;
    let my_public_key = my_keys.1;

    let sys_keys = crypting::gen_keys(passphrase.clone());
    let sys_private_key = sys_keys.0;
    let sys_public_key = sys_keys.1;

    // Writing keys to directory base-keys
    filesystem::echo(
        my_private_key,
        &filesystem::new_path("base-keys/my-key")
    );
    filesystem::echo(
        my_public_key,
        &filesystem::new_path("base-keys/my-key.pub")
    );

    filesystem::echo(
        sys_private_key,
        &filesystem::new_path("base-keys/sys-key")
    );
    filesystem::echo(
        sys_public_key,
        &filesystem::new_path("base-keys/sys-key.pub")
    );
    
    // Making directory contacts
    filesystem::mkAllDirs(
        &filesystem::new_path("contacts")
    );

    Ok(())
}
