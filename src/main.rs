use crc32fast::Hasher;
use std::env;

/// Generates a reproducible hex color string (e.g., "#RRGGBB") from an input string.
///
/// Uses the CRC32 hash of the input string to derive the RGB components.
///
/// # Arguments
///
/// * `input` - The input string slice to generate the color from.
///
/// # Returns
///
/// A `String` representing the hex color code.
fn string_to_hex_color(input: &str) -> String {
    let mut hasher = Hasher::new();

    hasher.update(input.as_bytes());
    let hash_value = hasher.finalize();

    // Extract RGB components from the hash bytes.
    // We use the lower 3 bytes (24 bits) of the 32-bit hash.
    // TODO: Introduce flags to control the strategy (for example, using other parts or combinations if you prefer different mappings)
    let r = (hash_value & 0xFF) as u8;        // Lowest byte
    let g = ((hash_value >> 8) & 0xFF) as u8; // Second byte
    let b = ((hash_value >> 16) & 0xFF) as u8;// Third byte

    // Format logic from AI-completion. Proceed with caution.
    // {:02X} formats the u8 as uppercase hex with leading zero if needed.
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // Use the first command-line argument as input
        let input_string = &args[1];
        let color = string_to_hex_color(input_string);
        println!("{}", color);
    } else {
        println!("Input is required."); // TODO: Provide an example.
    }
}
