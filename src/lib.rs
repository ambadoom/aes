pub mod aes;

pub fn ecb_encrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let ekey = aes::expand_key(key)?;
    let mut output = Vec::new();
    for block in data.chunks(16) {
        if block.len() != 16 {
            return Err("Size must be multiple of 16 bytes".to_string());
        }
        let encrypted = try!(aes::encrypt(&ekey, block));
        output.extend(encrypted.iter().cloned());
    }
    Ok(output)
}


pub fn ecb_decrypt(key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let ekey = aes::expand_key(key)?;
    let mut output = Vec::new();
    for block in data.chunks(16) {
        if block.len() != 16 {
            return Err("Size must be multiple of 16 bytes".to_string());
        }
        let decrypted = try!(aes::decrypt(&ekey, block));
        output.extend(decrypted.iter().cloned());
    }
    Ok(output)
}

