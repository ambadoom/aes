
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

fn invert_sbox(inbox: [u8; 256]) -> [u8; 256] {
    let mut outbox = [0; 256];
    for (i, v) in inbox.iter().enumerate() {
        outbox[*v as usize] = i as u8;
    }
    outbox
}


#[test]
fn generate_sbox_test() {
    for (i, v) in expand_key_128([0;16], &generate_sbox()).iter().enumerate() {
        print!("{:02x} ", v);
        if i % 16 == 15 {
            println!("");
        }
    }
}

fn schedule_core(bytes: &mut [u8; 4], iterations: u8, sbox: &[u8; 256]) {
    let fst = bytes[0];
    for i in 0..3 {
        bytes[i] = sbox[bytes[i+1] as usize];
    }
    bytes[3] = sbox[fst as usize];
    let mut rcon = 1;
    for i in 1..iterations {
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
    add_round_key(&mut state, &ekey[10*16 .. 11 * 16]);
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
    for (i, v) in encrypt_128(key, plain, &generate_sbox()).iter().enumerate() {
        print!("{:02x} ", v);
        if i % 16 == 15 {
            println!("");
        }
    }
}
 

