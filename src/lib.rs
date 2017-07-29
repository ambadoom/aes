pub mod aes;

pub fn ecb_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let ekey = aes::expand_key(key);
    let mut output = Vec::new();
    for block in data.chunks(16) {
        if block.len() != 16 {
            panic!("Size must be multiple of 16 bytes");
        }
        let encrypted = aes::encrypt(&ekey, block);
        output.extend(encrypted.iter().cloned());
    }
    output
}


pub fn ecb_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let ekey = aes::expand_key(key);
    let mut output = Vec::new();
    for block in data.chunks(16) {
        if block.len() != 16 {
            panic!("Size must be multiple of 16 bytes");
        }
        let decrypted = aes::decrypt(&ekey, block);
        output.extend(decrypted.iter().cloned());
    }
    output
}

