use std::num::ParseIntError;

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..=i + 1], 16))
        .collect()
}

const BASE64_CHARS: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '/', '+',
];

pub fn hex_to_base64(hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    debug_assert_eq!(hex.len() % 2, 0);
    let bytes = hex_to_bytes(hex)?;
    let mut bytes_ref = &bytes[..];
    let mut b64 = String::new();
    // can do 3 bytes at a time
    while bytes_ref.len() >= 3 {
        let (curr, rest) = bytes_ref.split_at(3);
        bytes_ref = rest;
        // first 6 bits
        let mut six = curr[0] >> 2;
        let mut two = curr[0] & 3;
        b64.push(BASE64_CHARS[six as usize]);
        let mut four = curr[1] >> 4;
        b64.push(BASE64_CHARS[((two << 4) | four) as usize]);
        four = curr[1] & 15;
        two = curr[2] >> 6;
        b64.push(BASE64_CHARS[((four << 2) | two) as usize]);
        six = curr[2] & 63;
        b64.push(BASE64_CHARS[six as usize]);
    }
    if !bytes_ref.is_empty() {
        if bytes_ref.len() == 2 {
            let six = bytes_ref[0] >> 2;
            let two = bytes_ref[0] & 3;
            b64.push(BASE64_CHARS[six as usize]);
            let mut four = bytes_ref[1] >> 4;
            b64.push(BASE64_CHARS[((two << 4) | four) as usize]);
            four = bytes_ref[1] & 15;
            b64.push(BASE64_CHARS[(four << 2) as usize]);
            b64.push('=');
        } else {
            let six = bytes_ref[0] >> 2;
            let two = bytes_ref[0] & 3;
            b64.push(BASE64_CHARS[six as usize]);
            b64.push(BASE64_CHARS[(two << 4) as usize]);
            b64.push_str("==");
        }
    }
    Ok(b64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bytes() {
        let hex = "373737";
        let bytes = vec![55, 55, 55];
        assert_eq!(bytes, hex_to_bytes(hex).expect("error"));
    }

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64, hex_to_base64(hex).expect("error"));
        let hex = format!("{:X}{:X}", 77, 97);
        let base64 = "TWE=";
        assert_eq!(base64, hex_to_base64(&hex).expect("error"));
        let hex = format!("{:X}", 77);
        let base64 = "TQ==";
        assert_eq!(base64, hex_to_base64(&hex).expect("error"));
    }
}
