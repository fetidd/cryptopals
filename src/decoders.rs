use crate::converters::hex_to_bytes;

pub fn fixed_xor(left: &str, right: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let left_bytes = hex_to_bytes(left)?;
    let right_bytes = hex_to_bytes(right)?;
    let result: Vec<_> = left_bytes
        .into_iter()
        .enumerate()
        .map(|(i, l)| l ^ right_bytes[i])
        .collect();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_xor() {
        let left = "1c0111001f010100061a024b53535009181c";
        let right = "686974207468652062756c6c277320657965";
        let exp = "746865206b696420646f6e277420706c6179";
        let act = fixed_xor(left, right)
            .unwrap()
            .into_iter()
            .map(|b| format!("{:X}", b))
            .collect::<Vec<_>>()
            .join("");
        assert_eq!(exp.to_lowercase(), act.to_ascii_lowercase());
    }
}
