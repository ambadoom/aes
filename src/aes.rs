
fn gf_multiply(a_in: u8, b_in: u8) -> u8 {
    let mut a = a_in;
    let mut b = b_in;
    let mut result = 0;
    for _ in 0..8 {
        if b & 1 != 0 {
            result ^= a;
        }
        let high_bit = a & 0x80;
        a <<= 1;
        b >>= 1;
        if high_bit != 0 {
            a ^= 0x1B;
        }
    }
    result
}

#[test]
fn gf_multiply_test() {
    assert_eq!(0x01, gf_multiply(0x03, 0xf6));
    assert_eq!(0x04, gf_multiply(0x02, 0x02));
}


const SBOX: [u8; 256] = 
    [0x63, 0x7C, 0x77, 0x7B, 0xF2, 0x6B, 0x6F, 0xC5, 0x30, 0x01, 0x67, 0x2B, 0xFE, 0xD7, 0xAB, 0x76
    ,0xCA, 0x82, 0xC9, 0x7D, 0xFA, 0x59, 0x47, 0xF0, 0xAD, 0xD4, 0xA2, 0xAF, 0x9C, 0xA4, 0x72, 0xC0
    ,0xB7, 0xFD, 0x93, 0x26, 0x36, 0x3F, 0xF7, 0xCC, 0x34, 0xA5, 0xE5, 0xF1, 0x71, 0xD8, 0x31, 0x15
    ,0x04, 0xC7, 0x23, 0xC3, 0x18, 0x96, 0x05, 0x9A, 0x07, 0x12, 0x80, 0xE2, 0xEB, 0x27, 0xB2, 0x75
    ,0x09, 0x83, 0x2C, 0x1A, 0x1B, 0x6E, 0x5A, 0xA0, 0x52, 0x3B, 0xD6, 0xB3, 0x29, 0xE3, 0x2F, 0x84
    ,0x53, 0xD1, 0x00, 0xED, 0x20, 0xFC, 0xB1, 0x5B, 0x6A, 0xCB, 0xBE, 0x39, 0x4A, 0x4C, 0x58, 0xCF
    ,0xD0, 0xEF, 0xAA, 0xFB, 0x43, 0x4D, 0x33, 0x85, 0x45, 0xF9, 0x02, 0x7F, 0x50, 0x3C, 0x9F, 0xA8
    ,0x51, 0xA3, 0x40, 0x8F, 0x92, 0x9D, 0x38, 0xF5, 0xBC, 0xB6, 0xDA, 0x21, 0x10, 0xFF, 0xF3, 0xD2
    ,0xCD, 0x0C, 0x13, 0xEC, 0x5F, 0x97, 0x44, 0x17, 0xC4, 0xA7, 0x7E, 0x3D, 0x64, 0x5D, 0x19, 0x73
    ,0x60, 0x81, 0x4F, 0xDC, 0x22, 0x2A, 0x90, 0x88, 0x46, 0xEE, 0xB8, 0x14, 0xDE, 0x5E, 0x0B, 0xDB
    ,0xE0, 0x32, 0x3A, 0x0A, 0x49, 0x06, 0x24, 0x5C, 0xC2, 0xD3, 0xAC, 0x62, 0x91, 0x95, 0xE4, 0x79
    ,0xE7, 0xC8, 0x37, 0x6D, 0x8D, 0xD5, 0x4E, 0xA9, 0x6C, 0x56, 0xF4, 0xEA, 0x65, 0x7A, 0xAE, 0x08
    ,0xBA, 0x78, 0x25, 0x2E, 0x1C, 0xA6, 0xB4, 0xC6, 0xE8, 0xDD, 0x74, 0x1F, 0x4B, 0xBD, 0x8B, 0x8A
    ,0x70, 0x3E, 0xB5, 0x66, 0x48, 0x03, 0xF6, 0x0E, 0x61, 0x35, 0x57, 0xB9, 0x86, 0xC1, 0x1D, 0x9E
    ,0xE1, 0xF8, 0x98, 0x11, 0x69, 0xD9, 0x8E, 0x94, 0x9B, 0x1E, 0x87, 0xE9, 0xCE, 0x55, 0x28, 0xDF
    ,0x8C, 0xA1, 0x89, 0x0D, 0xBF, 0xE6, 0x42, 0x68, 0x41, 0x99, 0x2D, 0x0F, 0xB0, 0x54, 0xBB, 0x16];

const INVBOX: [u8; 256] = 
    [0x52, 0x09, 0x6A, 0xD5, 0x30, 0x36, 0xA5, 0x38, 0xBF, 0x40, 0xA3, 0x9E, 0x81, 0xF3, 0xD7, 0xFB
    ,0x7C, 0xE3, 0x39, 0x82, 0x9B, 0x2F, 0xFF, 0x87, 0x34, 0x8E, 0x43, 0x44, 0xC4, 0xDE, 0xE9, 0xCB
    ,0x54, 0x7B, 0x94, 0x32, 0xA6, 0xC2, 0x23, 0x3D, 0xEE, 0x4C, 0x95, 0x0B, 0x42, 0xFA, 0xC3, 0x4E
    ,0x08, 0x2E, 0xA1, 0x66, 0x28, 0xD9, 0x24, 0xB2, 0x76, 0x5B, 0xA2, 0x49, 0x6D, 0x8B, 0xD1, 0x25
    ,0x72, 0xF8, 0xF6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xD4, 0xA4, 0x5C, 0xCC, 0x5D, 0x65, 0xB6, 0x92
    ,0x6C, 0x70, 0x48, 0x50, 0xFD, 0xED, 0xB9, 0xDA, 0x5E, 0x15, 0x46, 0x57, 0xA7, 0x8D, 0x9D, 0x84
    ,0x90, 0xD8, 0xAB, 0x00, 0x8C, 0xBC, 0xD3, 0x0A, 0xF7, 0xE4, 0x58, 0x05, 0xB8, 0xB3, 0x45, 0x06
    ,0xD0, 0x2C, 0x1E, 0x8F, 0xCA, 0x3F, 0x0F, 0x02, 0xC1, 0xAF, 0xBD, 0x03, 0x01, 0x13, 0x8A, 0x6B
    ,0x3A, 0x91, 0x11, 0x41, 0x4F, 0x67, 0xDC, 0xEA, 0x97, 0xF2, 0xCF, 0xCE, 0xF0, 0xB4, 0xE6, 0x73
    ,0x96, 0xAC, 0x74, 0x22, 0xE7, 0xAD, 0x35, 0x85, 0xE2, 0xF9, 0x37, 0xE8, 0x1C, 0x75, 0xDF, 0x6E
    ,0x47, 0xF1, 0x1A, 0x71, 0x1D, 0x29, 0xC5, 0x89, 0x6F, 0xB7, 0x62, 0x0E, 0xAA, 0x18, 0xBE, 0x1B
    ,0xFC, 0x56, 0x3E, 0x4B, 0xC6, 0xD2, 0x79, 0x20, 0x9A, 0xDB, 0xC0, 0xFE, 0x78, 0xCD, 0x5A, 0xF4
    ,0x1F, 0xDD, 0xA8, 0x33, 0x88, 0x07, 0xC7, 0x31, 0xB1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xEC, 0x5F
    ,0x60, 0x51, 0x7F, 0xA9, 0x19, 0xB5, 0x4A, 0x0D, 0x2D, 0xE5, 0x7A, 0x9F, 0x93, 0xC9, 0x9C, 0xEF
    ,0xA0, 0xE0, 0x3B, 0x4D, 0xAE, 0x2A, 0xF5, 0xB0, 0xC8, 0xEB, 0xBB, 0x3C, 0x83, 0x53, 0x99, 0x61
    ,0x17, 0x2B, 0x04, 0x7E, 0xBA, 0x77, 0xD6, 0x26, 0xE1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0C, 0x7D];

#[allow(dead_code)]
fn generate_sbox() -> [u8; 256] {
    let inv3: u8 = 0xf6;
    let mut up: u8 = 1;
    let mut down: u8 = 1;
    let mut outbox = [0; 256];

    while up != 1 || outbox[1] == 0 {
        up = gf_multiply(up, 0x03);
        down = gf_multiply(down, inv3);
        let mut value = down;
        value ^= down.rotate_left(1);
        value ^= down.rotate_left(2);
        value ^= down.rotate_left(3);
        value ^= down.rotate_left(4);
        value ^= 0x63;
        outbox[up as usize] = value;
    }
    outbox[0] = 0x63;
    outbox
}

#[allow(dead_code)]
fn invert_sbox(inbox: &[u8; 256]) -> [u8; 256] {
    let mut outbox = [0; 256];
    for (i, v) in inbox.iter().enumerate() {
        outbox[*v as usize] = i as u8;
    }
    outbox
}

#[test]
fn generate_sbox_test() {
    assert_eq!(SBOX[..], generate_sbox()[..]);
    assert_eq!(INVBOX[..], invert_sbox(&SBOX)[..]);
}

fn schedule_core(bytes: &mut [u8; 4], iterations: u8) {
    let fst = bytes[0];
    for i in 0..3 {
        bytes[i] = SBOX[bytes[i+1] as usize];
    }
    bytes[3] = SBOX[fst as usize];
    let mut rcon = 1;
    for _ in 1..iterations {
        rcon = gf_multiply(rcon, 2);
    }
    bytes[0] ^= rcon; 
}
    
pub fn expand_key(key: &[u8]) -> Result<Vec<u8>, &str> {
    let insize = key.len();
    let outsize = match insize {
        16 => 176,
        24 => 208,
        32 => 240,
        _  => return Err("Invalid key size"),
    };
    let mut tmp = [0; 4];
    let mut iterations = 1;
    let mut expanded: Vec<u8> = Vec::with_capacity(outsize);
    for byte in key.iter() {
        expanded.push(*byte);
    }
    let pull_tmp = |tmp: &mut [u8;4], expanded: &Vec<u8>| {
        for i in 0..4 {
            let pos = expanded.len();
            tmp[i] = expanded[pos + i - 4];
        }
    };
    let push_tmp = |tmp: &[u8;4], expanded: &mut Vec<u8>| {
        for i in 0..4 {
            let pos = expanded.len();
            let elem = expanded[pos-insize] ^ tmp[i];
            expanded.push(elem);
        }
    };

    while expanded.len() < outsize {
        pull_tmp(&mut tmp, &expanded);
        schedule_core(&mut tmp, iterations);
        iterations += 1;
        push_tmp(&tmp, &mut expanded);
        for _ in 0..3 {
            pull_tmp(&mut tmp, &expanded);
            push_tmp(&tmp, &mut expanded);
        }
        if insize == 32 {
            pull_tmp(&mut tmp, &expanded);
            for i in 0..4 {
                tmp[i] = SBOX[tmp[i] as usize];
            }
            push_tmp(&tmp, &mut expanded);
        }
        let steps = match insize { 24 => 2, 32 => 3, _ => 0 };
        for _ in 0..steps {
            pull_tmp(&mut tmp, &expanded);
            push_tmp(&tmp, &mut expanded);
        }
    }
    expanded.truncate(outsize);
    Ok(expanded)
}


pub fn encrypt(key: &[u8], block: &[u8]) ->  Result<[u8; 16], &'static str> {
    if block.len() != 16 {
        return Err("Invalid block size");
    }
    let mut state = [0; 16];
    for i in 0..16 {
        state[i] = block[i];
    }
    let ekey = match key.len() {
        16 | 24 | 32 => return encrypt(&expand_key(key).unwrap()[..], block),
        176 | 208 | 240 => key,
        _ => return Err("Invalid key size"),
    };
    let rounds = match ekey.len() {
        176 => 10,
        208 => 12,
        240 => 14,
        _ => panic!("Key expansion returned invalid size"),
    };
    add_round_key(&mut state, &ekey[0..16]);
    for round in 1..rounds {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &ekey[round*16 .. (round+1)*16]);
    }
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &ekey[rounds*16 .. (rounds+1)*16]);
    Ok(state)
}

fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
    for (i, byte) in round_key.iter().enumerate() {
        state[i] ^= *byte;
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for i in 0..16 {
        state[i] = SBOX[state[i] as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    let copy = state.clone();
    for i in 0..16 {
        let row = i % 4;
        let col = i / 4;
        let target = (row + col) % 4;
        state[i] = copy[target * 4 + row];
    }
}

fn mix_columns(state: &mut [u8; 16]) {
    for i in 0..4 {
        let b0 = state[4*i+0];
        let b1 = state[4*i+1];
        let b2 = state[4*i+2];
        let b3 = state[4*i+3];
        state[4*i+0] = gf_multiply(2, b0) ^ gf_multiply(3, b1) ^ b2 ^ b3;
        state[4*i+1] = b0 ^ gf_multiply(2, b1) ^ gf_multiply(3, b2) ^ b3;
        state[4*i+2] = b0 ^ b1 ^ gf_multiply(2, b2) ^ gf_multiply(3, b3);
        state[4*i+3] = gf_multiply(3, b0) ^ b1 ^ b2 ^ gf_multiply(2, b3);
    }
}
 
pub fn decrypt(key: &[u8], block: &[u8]) -> Result<[u8; 16], &'static str> {
    if block.len() != 16 {
        return Err("Invalid block size");
    }
    let mut state = [0; 16];
    for i in 0..16 {
        state[i] = block[i];
    }
    let ekey = match key.len() {
        16 | 24 | 32 => return decrypt(&expand_key(key).unwrap()[..], block),
        176 | 208 | 240 => key,
        _ => return Err("Invalid key size"),
    };
    let rounds = match ekey.len() {
        176 => 10,
        208 => 12,
        240 => 14,
        _ => panic!("Key expansion returned invalid size"),
    };
    add_round_key(&mut state, &ekey[rounds*16 .. (rounds+1)*16]);
    for round in (1..rounds).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &ekey[round*16 .. (round+1) * 16]);
        inv_mix_columns(&mut state);
    }
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &ekey[0..16]);
    Ok(state)
}

fn inv_sub_bytes(state: &mut [u8; 16]) {
    for i in 0..16 {
        state[i] = INVBOX[state[i] as usize];
    }
}

fn inv_shift_rows(state: &mut [u8; 16]) {
    let copy = state.clone();
    for i in 0..16 {
        let row = i % 4;
        let col = i / 4;
        let target = (4 + col - row) % 4;
        state[i] = copy[target * 4 + row];
    }
}

fn inv_mix_columns(state: &mut [u8; 16]) {
    for i in 0..4 {
        let b0 = state[4*i+0];
        let b1 = state[4*i+1];
        let b2 = state[4*i+2];
        let b3 = state[4*i+3];
        state[4*i+0] = gf_multiply(14, b0) ^ gf_multiply(11, b1) ^ gf_multiply(13, b2) ^ gf_multiply(9, b3);
        state[4*i+1] = gf_multiply(9, b0) ^ gf_multiply(14, b1) ^ gf_multiply(11, b2) ^ gf_multiply(13, b3);
        state[4*i+2] = gf_multiply(13, b0) ^ gf_multiply(9, b1) ^ gf_multiply(14, b2) ^ gf_multiply(11, b3);
        state[4*i+3] = gf_multiply(11, b0) ^ gf_multiply(13, b1) ^ gf_multiply(9, b2) ^ gf_multiply(14, b3);
    }
}

#[test]
fn encrypt_test_128() {
    let plain = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let key = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];
    let expected = [0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a];
    let actual = encrypt(&key, &plain).unwrap();
    assert_eq!(expected, actual);
    let reverse = decrypt(&key, &expected).unwrap();
    assert_eq!(plain, reverse);
}

#[test]
fn encrypt_test_192() {
    let plain = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let key = 
        [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f
        ,0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17];
    let expected = [0xdd, 0xa9, 0x7c, 0xa4, 0x86, 0x4c, 0xdf, 0xe0, 0x6e, 0xaf, 0x70, 0xa0, 0xec, 0x0d, 0x71, 0x91];
    let actual = encrypt(&key, &plain).unwrap();
    assert_eq!(expected, actual);
    let reverse = decrypt(&key, &expected).unwrap();
    assert_eq!(plain, reverse);
}

#[test]
fn encrypt_test_256() {
    let plain = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let key = 
        [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f
        ,0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f];
    let expected = [0x8e, 0xa2, 0xb7, 0xca, 0x51, 0x67, 0x45, 0xbf, 0xea, 0xfc, 0x49, 0x90, 0x4b, 0x49, 0x60, 0x89]; 
    let actual = encrypt(&key, &plain).unwrap();
    assert_eq!(expected, actual);
    let reverse = decrypt(&key, &expected).unwrap();
    assert_eq!(plain, reverse);
}
