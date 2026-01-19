use std::collections::VecDeque;

pub fn hex_to_base64(mut hex: &str) -> Result<String, Box<dyn std::error::Error>> {
    debug_assert_eq!(hex.len() % 2, 0);
    let mut bytes = VecDeque::new();
    while !hex.is_empty() {
        let (byte, rest) = hex.split_at(2);
        bytes.push_front(byte.parse::<u8>()?);
        hex = rest;
    }
    let mut base64 = String::new();
    while let Some(byte) = bytes.pop_front() {
        if bytes.is_empty() {
        } else {
        }
    }
    Ok(base64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(base64, hex_to_base64(hex).expect("error"));
    }
}
