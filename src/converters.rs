use std::{collections::VecDeque, num::ParseIntError};

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..=i + 1], 16))
        .collect()
}

pub fn hex_to_base64(mut hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    debug_assert_eq!(hex.len() % 2, 0);
    let bytes = hex_to_bytes(hex)?;
    dbg!(&bytes);
    let mut carry = 0;

    Ok(String::new())
}

// pub fn hex_to_base64_cheating(hex: &str) -> Result<String, Box<dyn std::error::Error>> {
//     Ok(BASE64_STANDARD.encode(hex::decode(hex)?))
// }

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
        // assert_eq!(base64, hex_to_base64_cheating(hex).expect("error"));
        assert_eq!(base64, hex_to_base64(hex).expect("error"));
    }
}
