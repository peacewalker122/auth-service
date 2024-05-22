// NOTE:
// The encoding process represents 40-bit groups of input bits as output strings of 8 encoded characters
// These 40 bits are then treated as 8 concatenated 5-bit groups, each of which is translated into a single character in the base 32 alphabet.

pub mod rand;

const BASE32_ALPHABET: [char; 32] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7',
];

pub fn encode_base32(val: &[u8], with_padding: bool) -> String {
    let mut result = String::new();
    let mut buf = 0_u64;
    let mut buf_length = 0_u32;

    // first step
    // for each byte or 8 bit buffer concat it to get only 5 bits avaible.
    for &byte in val {
        buf = (buf << 8) | byte as u64;
        buf_length += 8;

        while buf_length >= 5 {
            // proceed to take the 5 bit avaible in the buffer
            // from left to the right.
            let idx = (buf >> (buf_length - 5)) & 31;
            result.push(BASE32_ALPHABET[idx as usize]);
            buf_length -= 5;
        }
    }

    if buf_length > 0 {
        let idx = buf << (5 - buf_length) & 31;
        result.push(BASE32_ALPHABET[idx as usize]);
    }

    if with_padding {
        while result.len() % 8 != 0 {
            result.push('=')
        }
    }

    result
}

mod test {
    use super::encode_base32;

    // add code here
    #[test]
    fn test_ok_encode_base32() {
        let val = encode_base32(b"Hello World!", true);

        assert_eq!(val, "JBSWY3DPEBLW64TMMQQQ====");
    }
}
