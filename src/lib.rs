pub mod aes;

pub fn ecb_encrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let ekey = aes::expand_key(key);
    let mut output = Vec::new();
    for block in data.chunks(16) {
        if block.len() != 16 { panic!("Size must be multiple of 16 bytes"); }
        let encrypted = aes::encrypt(&ekey, block);
        output.extend(encrypted.iter().cloned());
    }
    output
}

pub fn ecb_decrypt(key: &[u8], data: &[u8]) -> Vec<u8> {
    let ekey = aes::expand_key(key);
    let mut output = Vec::new();
    for block in data.chunks(16) {
        if block.len() != 16 { panic!("Size must be multiple of 16 bytes"); }
        let decrypted = aes::decrypt(&ekey, block);
        output.extend(decrypted.iter().cloned());
    }
    output
}

#[test]
fn ecb_test() {
    let data = [1; 128];
    let key = [2; 16];
    let encrypted = ecb_encrypt(&key, &data);
    let decrypted = ecb_decrypt(&key, &encrypted);
    assert_eq!(&data[..], &decrypted[..]);
}

pub fn cbc_encrypt(key: &[u8], data: &[u8], iv: &[u8]) -> Vec<u8> {
    let ekey = aes::expand_key(key);
    let mut output = Vec::new();
    let mut previous = [0; 16];
    for i in 0..16 { previous[i] = iv[i]; }
    for block in data.chunks(16) {
        if block.len() != 16 { panic!("Size must be multiple of 16 bytes"); }
        for i in 0..16 {
            previous[i] ^= block[i];
        }
        let encrypted = aes::encrypt(&ekey, &previous);
        previous = encrypted;
        output.extend(encrypted.iter().cloned());
    }
    output
}

pub fn cbc_decrypt(key: &[u8], data: &[u8], iv: &[u8]) -> Vec<u8> {
    let ekey = aes::expand_key(key);
    let mut output = Vec::new();
    let mut previous = [0; 16];
    for i in 0..16 { previous[i] = iv[i]; }
    for block in data.chunks(16) {
        if block.len() != 16 { panic!("Size must be multiple of 16 bytes"); }
        let mut decrypted = aes::decrypt(&ekey, block);
        for i in 0..16 {
            decrypted[i] ^= previous[i];
            previous[i] = block[i];
        }
        output.extend(decrypted.iter().cloned());
    }
    output
}

#[test]
fn cbc_test() {
    let data = [3; 128];
    let key = [4; 16];
    let iv = [5; 16];
    let encrypted = cbc_encrypt(&key, &data, &iv);
    let decrypted = cbc_decrypt(&key, &encrypted, &iv);
    assert_eq!(&data[..], &decrypted[..]);
}

pub fn pad(data: &[u8]) -> Vec<u8> {
    let length = data.len();
    let target = length - length % 16 + 16;
    let padding = target - length;
    let mut output = data.to_vec();
    for _ in 0..padding {
        output.push(padding as u8);
    }
    output
}

#[test]
fn pad_test() {
    let data = [7;11];
    let padded = pad(&data);
    assert_eq!([7,7,7,7,7,7,7,7,7,7,7,5,5,5,5,5], padded[..]);
    let data2 = [4;16];
    let padded2 = pad(&data2);
    assert_eq!([4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16,16], padded2[..]);
    // ^ allowing for no padding at all creates ambiguity if the unpadded data has an ending that
    // looks like it could be padding so we pad for a full block
}
