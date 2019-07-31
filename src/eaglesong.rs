use crate::const_vars::*;

fn eaglesong_permutation(state : &mut [u32]) {
    let mut new = [0 as u32; 16];

    for i in 0..NUM_ROUNDS {
        // bit matrix
        for j in 0..16 {
            new[j] = 0;
            for k in 0..16 {
                new[j] = new[j] ^ (BIT_MATRIX[k * 16 + j].wrapping_mul(state[k]));
            }
        }
        for j in 0..16 {
            state[j] = new[j];
        }

        // circulant multiplication
        for j in 0..16 {
            state[j] = state[j] ^ state[j].rotate_left(COEFFICIENTS[3 * j + 1]) ^ state[j].rotate_left(COEFFICIENTS[ 3 * j + 2]);
        }

        // constants injection
        for j in 0..16 {
            state[j] = state[j] ^ INJECTION_CONSTANTS[i * 16 + j];
        }

        // addition / rotation / addition
        for j in (0..16).step_by(2) {
            state[j] = state[j].wrapping_add(state[j + 1]);
            state[j] = state[j].rotate_left(8);
            state[j+1] = state[j+1].rotate_left(24);
            state[j+1] = state[j].wrapping_add(state[j+1]);
        }
    }
}

pub fn eaglesong_sponge(output: &mut [u8], output_length : usize, input: &[u8], input_length : usize, delimiter : u8) {
    let mut state = [0 as u32; 16];

    // initialize to zero
    for i in 0..16 {
        state[i] = 0;
    }

    // absorbing
    for i in 0..(((input_length + 1) * 8 + RATE - 1) / RATE) {
        for j in 0..(RATE / 32) {
            let mut integer : u32 = 0;
            for k in 0..4 {
                if i * RATE / 8 + j * 4 + k < input_length {
                    integer = (integer << 8) ^ (input[i * RATE / 8 + j * 4 + k] as u32);
                }else if i* RATE / 8 + j * 4 + k == input_length {
                    integer = (integer << 8) ^ (delimiter as u32);
                }
            }
            state[j] = state[j] ^ integer;
        }
        eaglesong_permutation(&mut state);
    }

    // squeezing
    for i in 0..(output_length / (RATE / 8)) {
        for j in 0..(RATE / 32) {
            for k in 0..4 {
                output[i * RATE / 8 + j * 4 + k] = ((state[j] >> ( 8 * k as u32)) & 0xff) as u8;
            }
        }
        eaglesong_permutation(&mut state);
    }
}

pub fn eaglesong_update(state: &mut [u32; 16], input: &[u8]) {
    for i in 0..(input.len() * 8 / RATE) {
        for j in 0..(RATE / 32) {
            let mut integer : u32 = 0;
            for k in 0..4 {
                integer = (integer << 8) ^ (input[i * RATE / 8 + j * 4 + k] as u32);
            }
            state[j] = state[j] ^ integer;
        }
        eaglesong_permutation(state);
    }
}

pub fn eaglesong_finalize(state: &mut [u32; 16], input: &[u8], output: &mut [u8], output_length : usize) {
    for j in 0..(RATE / 32) {
        let mut integer : u32 = 0;
        for k in 0..4 {
            if j * 4 + k < input.len() {
                integer = (integer << 8) ^ (input[j * 4 + k] as u32);
            }else if j * 4 + k == input.len() {
                integer = (integer << 8) ^ (DELIMITER as u32);
            }
        }
        state[j] = state[j] ^ integer;
    }
    eaglesong_permutation(state);

    for i in 0..(output_length / (RATE / 8)) {
        for j in 0..(RATE / 32) {
            for k in 0..4 {
                output[i * RATE / 8 + j * 4 + k] = ((state[j] >> ( 8 * k as u32)) & 0xff) as u8;
            }
        }
        eaglesong_permutation(state);
    }
}