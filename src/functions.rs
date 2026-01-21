use std::num::ParseIntError;

pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..=i + 1], 16))
        .collect()
}

pub const BASE64_CHARS: [char; 64] = [
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

pub fn fixed_xor(left: &str, right: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let left_bytes = hex_to_bytes(left)?;
    let result: Vec<_> = left_bytes
        .into_iter()
        .enumerate()
        .map(|(i, l)| l ^ right[i])
        .collect();
    Ok(result)
}

fn get_letter_frequency(char: char) -> Option<f32> {
    match char.to_ascii_lowercase() {
        'a' => Some(0.082),
        'b' => Some(0.015),
        'c' => Some(0.028),
        'd' => Some(0.043),
        'e' => Some(0.127),
        'f' => Some(0.022),
        'g' => Some(0.02),
        'h' => Some(0.061),
        'i' => Some(0.07),
        'j' => Some(0.0016),
        'k' => Some(0.0077),
        'l' => Some(0.04),
        'm' => Some(0.024),
        'n' => Some(0.067),
        'o' => Some(0.075),
        'p' => Some(0.019),
        'q' => Some(0.0012),
        'r' => Some(0.06),
        's' => Some(0.063),
        't' => Some(0.091),
        'u' => Some(0.028),
        'v' => Some(0.0098),
        'w' => Some(0.024),
        'x' => Some(0.0015),
        'y' => Some(0.02),
        'z' => Some(0.00074),
        _ => None,
    }
}

pub const ALPHABET_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

fn calculate_fitting_quotient(string: &str) -> f32 {
    let mut fq = 0.0;
    let l = string.len() as f32;
    for letter in ALPHABET_LOWER {
        let appearing = string.chars().fold(0, |acc, ch| {
            if ch.to_ascii_lowercase() == letter {
                acc + 1
            } else {
                acc
            }
        });
        let normal_freq = get_letter_frequency(letter).expect("passed in a non-letter");
        let diff = (normal_freq - (appearing as f32 / l)).abs();
        fq += diff;
    }
    fq / ALPHABET_LOWER.len() as f32
}

pub fn decipher_single_byte_xor(
    input: &str,
) -> Result<(u8, String, f32), Box<dyn std::error::Error>> {
    let mut best_result: Option<(u8, String, f32)> = None;
    let mut lowest_fitting_quotient = 1.0;
    let mut fqs = vec![];
    for ch in 0..=255 {
        let xor = vec![ch; input.len()];
        if let Ok(xored) = fixed_xor(input, &xor) {
            let output = xored
                .into_iter()
                .map(|b| (b as char).to_string())
                .collect::<Vec<_>>()
                .join("");
            let fitting_quotient = calculate_fitting_quotient(&output);
            fqs.push((output.clone(), fitting_quotient));
            if fitting_quotient < lowest_fitting_quotient {
                lowest_fitting_quotient = fitting_quotient;
                best_result = Some((ch, output, fitting_quotient));
            }
        }
    }
    fqs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    fqs.truncate(10);
    dbg!(fqs);
    let mut selected_fqs = vec![];
    for fq in fqs.into_iter() {
        // only select fqs with >95% letter appearance
    }
    if let Some(output) = best_result {
        Ok(output)
    } else {
        Err("failed to decipher!".into())
    }
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

    #[test]
    fn test_fixed_xor() {
        let left = "1c0111001f010100061a024b53535009181c";
        let right = "686974207468652062756c6c277320657965";
        let exp = "746865206b696420646f6e277420706c6179";
        let act = fixed_xor(left, &hex_to_bytes(right).unwrap())
            .unwrap()
            .into_iter()
            .map(|b| format!("{:X}", b))
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(exp.to_lowercase(), act.to_ascii_lowercase());
    }

    #[test]
    fn test_decipher_single_byte_xor() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected = (
            88,
            "Cooking MC's like a pound of bacon".to_string(),
            0.027448822,
        );
        assert_eq!(expected, decipher_single_byte_xor(input).unwrap());
    }
}
