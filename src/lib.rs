extern crate crunchy;

use crate::const_vars::*;
use crate::eaglesong::*;
use std::vec::Vec;

mod const_vars;
mod eaglesong;

pub fn eaglesong(input: &[u8], output: &mut [u8]) {
    eaglesong_sponge(output, output.len(), input, input.len(), DELIMITER)
}

pub struct EagleSongBuilder {
    state: [u32; 16],
    length: usize,
    msg: Vec<u8>,
}

impl EagleSongBuilder {
    pub fn new() -> Self {
        EagleSongBuilder {
            state: [0 as u32; 16],
            length: 0,
            msg: Vec::new(),
        }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        let bytes_len = bytes.len();
        self.length += bytes_len;
        self.msg.extend_from_slice(bytes);

        eaglesong_update(&mut self.state, &self.msg);
        let rem_len = self.length % (RATE / 8);
        let rem_msg = self.msg.split_off(self.length - rem_len);
        self.msg = rem_msg;
        self.length = rem_len;
    }

    pub fn finalize(&mut self) -> [u8; 32] {
        let mut output = [0 as u8; 32];
        eaglesong_finalize(&mut self.state, &self.msg, &mut output, 32);
        return output;
    }
}

#[cfg(test)]
mod test {
    use crate::eaglesong;
    use crate::EagleSongBuilder;

    // 9e4452fc7aed93d7240b7b55263792befd1be09252b456401122ba71a56f62a0
    pub const BLANK_HASH: [u8; 32] = [
        158, 68, 82, 252, 122, 237, 147, 215, 36, 11, 123, 85, 38, 55, 146, 190, 253, 27, 224, 146,
        82, 180, 86, 64, 17, 34, 186, 113, 165, 111, 98, 160,
    ];
    // a50a3310f78cbaeadcffe2d46262119eeeda9d6568b4df1b636399742c867aca
    pub const HASH_34_1: [u8; 32] = [
        165, 10, 51, 16, 247, 140, 186, 234, 220, 255, 226, 212, 98, 98, 17, 158, 238, 218, 157,
        101, 104, 180, 223, 27, 99, 99, 153, 116, 44, 134, 122, 202,
    ];
    // d7b68a8ba4cfc606e86ed7e2ca3244ff27b28bb24c28ee09af4b1f47af9b9236
    pub const HASH_32_1_2_2: [u8; 32] = [
        215, 182, 138, 139, 164, 207, 198, 6, 232, 110, 215, 226, 202, 50, 68, 255, 39, 178, 139,
        178, 76, 40, 238, 9, 175, 75, 31, 71, 175, 155, 146, 54,
    ];
    #[test]
    fn empty_eaglesong() {
        let mut output = [0 as u8; 32];
        let input = [];
        eaglesong(&input, &mut output);
        assert_eq!(output, BLANK_HASH);
    }
    #[test]
    fn simple() {
        let mut output = [0 as u8; 32];
        let mut input = "1111111111111111111111111111111111\n".as_bytes();
        eaglesong(&input, &mut output);
        assert_eq!(output, HASH_34_1);

        input = "1111111111111111111111111111111122\n".as_bytes();
        eaglesong(&input, &mut output);
        assert_eq!(output, HASH_32_1_2_2);
    }

    #[test]
    fn builder_test() {
        let mut eaglesong_builder = EagleSongBuilder::new();
        assert_eq!(eaglesong_builder.finalize(), BLANK_HASH);

        let mut eaglesong_builder_1 = EagleSongBuilder::new();
        eaglesong_builder_1.update("111111111111111111111111".as_bytes());
        eaglesong_builder_1.update("1111111111\n".as_bytes());
        assert_eq!(eaglesong_builder_1.finalize(), HASH_34_1);

        let mut eaglesong_builder_2 = EagleSongBuilder::new();
        eaglesong_builder_2.update("11111111111111111111111111111111".as_bytes());
        eaglesong_builder_2.update("11\n".as_bytes());
        assert_eq!(eaglesong_builder_2.finalize(), HASH_34_1);
    }
}
