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
    let bytes = hex_to_bytes(hex)?;
    bytes_to_base64(&bytes)
}

pub fn bytes_to_base64(bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
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

pub fn fixed_xor(left: &[u8], right: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let result: Vec<_> = left
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

fn calculate_fitting_quotient(bytes: &[u8]) -> f32 {
    let mut fq = 0.0;
    let l = bytes.len() as f32;
    for letter in ALPHABET_LOWER {
        let appearing = bytes.iter().filter(|b| b.to_ascii_lowercase() == letter as u8).count();
        let normal_freq = get_letter_frequency(letter).expect("passed in a non-letter");
        let diff = (normal_freq - (appearing as f32 / l)).abs();
        fq += diff;
    }
    fq / ALPHABET_LOWER.len() as f32
}

pub fn break_single_byte_xor(input: &[u8]) -> Result<(u8, Vec<u8>, f32), Box<dyn std::error::Error>> {
    let mut fqs = vec![];
    for ch in 0..=255 {
        let xor = vec![ch; input.len()];
        if let Ok(xored) = fixed_xor(input, &xor) {
            let fitting_quotient = calculate_fitting_quotient(&xored);
            fqs.push((ch, xored, fitting_quotient));
        }
    }
    fqs.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    fqs.truncate(10);
    let mut selected_fqs = vec![];
    for fq in fqs.into_iter() {
        let fq_len = fq.1.len() as f32;
        let alphas = fq.1.iter().filter(|b: &&u8| {
            b.is_ascii_alphabetic() || **b == b' '
        }).count();
        let alphas = alphas as f32 / fq_len;
        if alphas > 0.95 {
            selected_fqs.push(fq);
        }
    }
    if !selected_fqs.is_empty() {
        let selected = selected_fqs.remove(0);
        Ok(selected)
    } else {
        Err("failed to decipher!".into())
    }
}

pub fn encrypt_repeating_key_xor(input: &[u8], key: &str) -> Vec<u8> {
    let mut xor_byte = key.bytes().cycle();
    input
        .iter()
        .map(|b| b ^ xor_byte.next().unwrap())
        .collect()
}

pub fn decrypt_repeating_key_xor(input: &[u8], key: &str) -> Vec<u8> {
    encrypt_repeating_key_xor(input, key)
}

pub fn calculate_edit_distance_single_byte(l: u8, r: u8) -> usize {
    let xored = l ^ r;
    [1, 2, 4, 8, 16, 32, 64, 128]
        .into_iter()
        .filter(|b| b & xored == *b)
        .count()
}

pub fn calculate_edit_distance(left: &[u8], right: &[u8]) -> usize {
    left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| calculate_edit_distance_single_byte(*l, *r))
        .sum()
}

pub fn base64_to_bytes(input: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut bytes = Vec::new();
    let input: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let chars: Vec<char> = input.chars().collect();
    for chunk in chars.chunks(4) {
        let indices: Vec<u8> = chunk
            .iter()
            .filter(|&&c| c != '=')
            .map(|&c| match c {
                'A'..='Z' => c as u8 - b'A',
                'a'..='z' => c as u8 - b'a' + 26,
                '0'..='9' => c as u8 - b'0' + 52,
                '+' => 62,
                '/' => 63,
                _ => panic!("invalid base64 character: {c}"),
            })
            .collect();
        match indices.len() {
            4 => {
                bytes.push((indices[0] << 2) | (indices[1] >> 4));
                bytes.push((indices[1] << 4) | (indices[2] >> 2));
                bytes.push((indices[2] << 6) | indices[3]);
            }
            3 => {
                bytes.push((indices[0] << 2) | (indices[1] >> 4));
                bytes.push((indices[1] << 4) | (indices[2] >> 2));
            }
            2 => {
                bytes.push((indices[0] << 2) | (indices[1] >> 4));
            }
            _ => return Err("invalid base64 input".into()),
        }
    }
    Ok(bytes)
}

pub fn break_repeating_key_xor(mut input: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    // assert!(input.len() % 4 == 0); // this is base64 encoded so should be a multiple of 4
    let mut keys = vec![];
    for keysize in 1..=40 {
        if keysize > input.len()/2 {
            break;
        }
        let left = &input[..keysize];
        let right = &input[keysize..keysize * 2];
        let distance = calculate_edit_distance(left, right);
        let normalized: f32 = distance as f32 / keysize as f32;
        keys.push((keysize, normalized));
    }
    keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let keysize = keys[0].0; // TODO this should be more keysizes, maybe 2/3
    let mut chunks = vec![];
    while input.len() >= keysize {
        let (chunk, rest) = input.split_at(keysize);
        chunks.push(chunk);
        input = rest;
    }
    chunks.push(input);
    let chunks = transpose(&chunks);
    let mut key = vec![];
    for chunk in chunks {
        let (byte, _, _) = break_single_byte_xor(&chunk)?;
        key.push(byte);
    }
    Ok(String::from_utf8(key)?)
}

fn transpose(chunks: &[&[u8]]) -> Vec<Vec<u8>> {
    let mut transposed = vec![vec![]; chunks[0].len()];
    for chunk in chunks {
        for (i, byte) in chunk.iter().enumerate() {
            transposed[i].push(*byte);
        }
    }
    transposed
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transpose() {
        let chunks = vec![b"abc".as_slice(), b"def".as_slice(), b"ghi".as_slice()];
        let expected = vec![b"adg".as_slice(), b"beh".as_slice(), b"cfi".as_slice()];
        let result = transpose(&chunks);
        assert_eq!(result, expected);
    }
    
    #[test]
    fn test_calculate_fitting_quotient() {
        let input = b"The quick brown fox jumps over the lazy dog. This sentence is a well-known pangram, yet it does not actually reflect the standard distribution of letters in the English language. Instead, a natural text, such as this one, with varied words and a reasonable length, is better for testing frequency, as E, T, A, and O are most common.";
        let expected = 0.009370335;
        let result = calculate_fitting_quotient(input);
        assert_eq!(result, expected);
    }

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
    fn test_base64_to_bytes() {
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let bytes = "I'm killing your brain like a poisonous mushroom".as_bytes();
        assert_eq!(bytes, base64_to_bytes(base64).expect("error"));
        
        let base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let bytes = hex_to_bytes(hex).unwrap();
        assert_eq!(bytes, base64_to_bytes(base64).expect("error"));
    }

    #[test]
    fn test_fixed_xor() {
        let left = "1c0111001f010100061a024b53535009181c";
        let right = "686974207468652062756c6c277320657965";
        let exp = "746865206b696420646f6e277420706c6179";
        let act = fixed_xor(&hex_to_bytes(left).unwrap(), &hex_to_bytes(right).unwrap())
            .unwrap()
            .into_iter()
            .map(|b| format!("{:X}", b))
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(exp.to_lowercase(), act.to_ascii_lowercase());
    }

    #[test]
    fn test_break_single_byte_xor() {
        let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let expected = "Cooking MC's like a pound of bacon".as_bytes();
        assert_eq!(expected, break_single_byte_xor(&hex_to_bytes(input).unwrap()).unwrap().1);
    }

    #[test]
    fn test_encrypt_decrypt_repeating_key_xor() {
        let plaintext = "Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal";
        let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f"
            .to_string();
        let encrypted = encrypt_repeating_key_xor(plaintext.as_bytes(), "ICE");
        let encrypted_hex = bytes_to_hex(&encrypted);
        assert_eq!(expected, encrypted_hex);
        let decrypted = decrypt_repeating_key_xor(&hex_to_bytes(&expected).unwrap(), "ICE");
        assert_eq!(plaintext, String::from_utf8(decrypted).unwrap());
        let decrypted = decrypt_repeating_key_xor(&encrypted, "ICE");
        assert_eq!(plaintext, String::from_utf8(decrypted).unwrap());
    }

    #[test]
    fn test_calculate_edit_distance_single_byte() {
        for (l, r, exp) in vec![(1, 1, 0), (2, 2, 0), (1, 3, 1), (1, 2, 2)] {
            assert_eq!(exp, calculate_edit_distance_single_byte(l, r));
        }
    }
    
    #[test]
    fn test_calculate_edit_distance() {
        let left = "this is a test";
        let right = "wokka wokka!!!";
        let expected = 37;
        assert_eq!(expected, calculate_edit_distance(left.as_bytes(), right.as_bytes()));
    }

    #[test]
    fn test_break_repeating_key_xor() {
//         let expected = "Burning 'em, if you ain't quick and nimble
// I go crazy when I hear a cymbal";
//         let input = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
//         assert_eq!(expected, String::from_utf8(break_repeating_key_xor(&hex_to_bytes(input).unwrap()).unwrap()).unwrap());
        
        let file = include_str!("../examples/repeating_key_xor_encrypted.txt");
        let file = base64_to_bytes(file).unwrap();
        let result = break_repeating_key_xor(&file).unwrap();
        assert_eq!("TODO", result);
    }
}
