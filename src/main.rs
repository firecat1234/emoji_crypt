use std::env;
use std::process;

/// Maps a nibble (0–15) to its corresponding Unicode variation selector.
/// Here we map:
///  0 → U+FE00, 1 → U+FE01, …, 15 → U+FE0F.
fn nibble_to_variation(nibble: u8) -> char {
    // Assumes nibble is in 0..=15.
    std::char::from_u32(0xFE00 + nibble as u32).unwrap()
}

/// Converts a variation selector (U+FE00 to U+FE0F) back to its 4‑bit value.
fn variation_to_nibble(vs: char) -> Option<u8> {
    let code = vs as u32;
    if (0xFE00..=0xFE0F).contains(&code) {
        Some((code - 0xFE00) as u8)
    } else {
        None
    }
}

/// Encodes a secret message into emoji pairs.
/// Each byte is split into two 4‑bit nibbles. For each nibble, the function outputs:
///   • the base emoji (e.g. 😀) and
///   • a variation selector corresponding to that nibble (from U+FE00 to U+FE0F).
fn encode(secret: &str, base_emoji: char) -> String {
    let mut result = String::new();
    for byte in secret.as_bytes() {
        let high = byte >> 4;
        let low = byte & 0x0F;
        // Encode high nibble
        result.push(base_emoji);
        result.push(nibble_to_variation(high));
        // Encode low nibble
        result.push(base_emoji);
        result.push(nibble_to_variation(low));
    }
    result
}

/// Decodes an encoded string by extracting variation selectors,
/// converting each pair of selectors back into a byte.
fn decode(encoded: &str) -> Result<String, String> {
    // Filter the string to extract only variation selectors (U+FE00–U+FE0F)
    let selectors: Vec<char> = encoded.chars()
        .filter(|&c| (0xFE00..=0xFE0F).contains(&(c as u32)))
        .collect();
    
    if selectors.len() % 2 != 0 {
        return Err("Encoded message has an incomplete nibble sequence.".to_string());
    }
    
    let mut bytes = Vec::new();
    for chunk in selectors.chunks(2) {
        let high = variation_to_nibble(chunk[0])
            .ok_or_else(|| "Invalid variation selector found.".to_string())?;
        let low = variation_to_nibble(chunk[1])
            .ok_or_else(|| "Invalid variation selector found.".to_string())?;
        let byte = (high << 4) | low;
        bytes.push(byte);
    }
    
    String::from_utf8(bytes).map_err(|e| format!("UTF-8 conversion error: {}", e))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <encode|decode> <message> [base_emoji]", args[0]);
        process::exit(1);
    }
    let command = &args[1];
    if command == "encode" {
        let secret = &args[2];
        // Use the provided base emoji or default to 😀.
        let base_emoji = if args.len() > 3 {
            args[3].chars().next().unwrap_or('😀')
        } else {
            '😀'
        };
        let encoded = encode(secret, base_emoji);
        println!("{}", encoded);
    } else if command == "decode" {
        let encoded = &args[2];
        match decode(encoded) {
            Ok(secret) => println!("{}", secret),
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("Unknown command: {}", command);
        process::exit(1);
    }
}
