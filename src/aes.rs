
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

fn generate_sbox() -> [u8; 256] {
    let inv3: u8 = 0xf6;
    let mut up: u8 = 1;
    let mut down: u8 = 1;
    let mut sbox = [0; 256];

    while up != 1 || sbox[1] == 0 {
        up = gf_multiply(up, 0x03);
        down = gf_multiply(down, inv3);
        let mut value = down;
        value ^= down.rotate_left(1);
        value ^= down.rotate_left(2);
        value ^= down.rotate_left(3);
        value ^= down.rotate_left(4);
        value ^= 0x63;
        sbox[up as usize] = value;
    }
    sbox[0] = 0x63;
    sbox
}

fn invert_sbox(inbox: &[u8; 256]) -> [u8; 256] {
    let mut outbox = [0; 256];
    for (i, v) in inbox.iter().enumerate() {
        outbox[*v as usize] = i as u8;
    }
    outbox
}


#[test]
fn generate_sbox_test() {
    let expected: [u8; 256] = 
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
    assert_eq!(expected[..], generate_sbox()[..]);
}

fn schedule_core(bytes: &mut [u8; 4], iterations: u8, sbox: &[u8; 256]) {
    let fst = bytes[0];
    for i in 0..3 {
        bytes[i] = sbox[bytes[i+1] as usize];
    }
    bytes[3] = sbox[fst as usize];
    let mut rcon = 1;
    for _ in 1..iterations {
        rcon = gf_multiply(rcon, 2);
    }
    bytes[0] ^= rcon; 
}
    

fn expand_key_128(key: [u8; 16], sbox: &[u8; 256]) -> [u8; 176] {
    let mut tmp = [0; 4];
    let mut count = 16;
    let mut iterations = 1;
    let mut expanded: [u8; 176] = [0; 176];
    for (index, byte) in key.iter().enumerate() {
        expanded[index] = *byte;
    }

    while count < 176 {
        for i in 0..4 {
            tmp[i] = expanded[count + i - 4];
        }
        if count % 16 == 0 {
            schedule_core(&mut tmp, iterations, sbox);
            iterations += 1;
        }
        for i in 0..4 {
            expanded[count] = expanded[count-16] ^ tmp[i];
            count += 1;
        }
    }
    expanded
}

fn encrypt_128(key: [u8; 16], block: [u8; 16], sbox: &[u8; 256]) -> [u8; 16] {
    let mut state = block;
    let ekey = expand_key_128(key, sbox);
    add_round_key(&mut state, &ekey[0..16]);
    for round in 1..10 {
        sub_bytes(&mut state, sbox);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &ekey[round*16 .. (round+1)*16]);
    }
    sub_bytes(&mut state, sbox);
    shift_rows(&mut state);
    add_round_key(&mut state, &ekey[10*16 .. 11*16]);
    state
}

fn add_round_key(state: &mut [u8; 16], round_key: &[u8]) {
    for (i, byte) in round_key.iter().enumerate() {
        state[i] ^= *byte;
    }
}

fn sub_bytes(state: &mut [u8; 16], sbox: &[u8; 256]) {
    for i in 0..16 {
        state[i] = sbox[state[i] as usize];
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
#[test]
fn encrypt_test() {
    let plain = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff];
    let key = [0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f];
    let expected = [0x69, 0xc4, 0xe0, 0xd8, 0x6a, 0x7b, 0x04, 0x30, 0xd8, 0xcd, 0xb7, 0x80, 0x70, 0xb4, 0xc5, 0x5a];
    let actual = encrypt_128(key, plain, &generate_sbox());
    assert_eq!(expected, actual);
    let reverse = decrypt_128(key, expected, &generate_sbox());
    assert_eq!(plain, reverse);
}
 
fn decrypt_128(key: [u8; 16], block: [u8; 16], sbox: &[u8; 256]) -> [u8; 16] {
    let mut state = block;
    let invbox = &invert_sbox(sbox);
    let ekey = expand_key_128(key, sbox);
    add_round_key(&mut state, &ekey[10*16 .. 11*16]);
    for round in (1..10).rev() {
        inv_shift_rows(&mut state);
        sub_bytes(&mut state, invbox);
        add_round_key(&mut state, &ekey[round*16 .. (round+1) * 16]);
        inv_mix_columns(&mut state);
    }
    inv_shift_rows(&mut state);
    sub_bytes(&mut state, invbox);
    add_round_key(&mut state, &ekey[0..16]);
    state
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
