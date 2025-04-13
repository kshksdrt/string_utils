use crate::utils::hash_file_path;
use color_art::Color;

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
pub(crate) fn string_to_hex_color(input: &str) -> String {
    let hash_value = hash_file_path(input);

    // Extract RGB components from the hash bytes.
    // We use the lower 3 bytes (24 bits) of the 32-bit hash.
    let r = (hash_value & 0xFF) as u8; // Lowest byte
    let g = ((hash_value >> 8) & 0xFF) as u8; // Second byte
    let b = ((hash_value >> 16) & 0xFF) as u8; // Third byte

    // Format logic from AI-completion. Proceed with caution.
    // {:02X} formats the u8 as uppercase hex with leading zero if needed.
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Generates a reproducible HSL color string (e.g., "hsl(120, 100%, 50%)") from an input string.
///
/// Uses the CRC32 hash of the input string to derive the Hue component.
/// Saturation and Lightness are fixed for vibrant and consistent colors.
///
/// # Arguments
///
/// * `input` - The input string slice to generate the color from.
///
/// # Returns
///
/// A `String` representing the HSL color code.
pub(crate) fn string_to_hex_color_sl(
    input: &str,
    saturation_percentage_0_to_1: f64,
    lightness_percentage_0_to_1: f64,
) -> String {
    let hash_value = hash_file_path(input);

    // Hue (H): Map a byte of the hash (0-255) to the Hue range (0-360 degrees).
    // We'll use the lowest byte here.
    let hue_byte = (hash_value & 0xFF) as u8;

    // Convert byte range to degree range (0.0 to 359.0).
    // Use f64 for precision needed by the color library.
    // .min(359.0) prevents potential rounding issues yielding 360.0
    let hue = (hue_byte as f64 / 255.0 * 360.0).round().min(359.0);

    // --- Convert HSL to RGB and Format as Hex ---

    // Use the color_art crate to create a Color object from HSL values.
    // Note: HSL values for the crate are H:[0.0, 360.0), S:[0.0, 1.0], L:[0.0, 1.0]
    match Color::from_hsl(
        hue,
        saturation_percentage_0_to_1 as f64,
        lightness_percentage_0_to_1 as f64,
    ) {
        Ok(color) => {
            // Convert the color object to a hex string (#RRGGBB).
            color.hex_full()
        }
        Err(e) => {
            eprintln!(
                "Failed to create color from HSL(h:{}, s:{}, l:{}) for input '{}': {}",
                hue, saturation_percentage_0_to_1, lightness_percentage_0_to_1, input, e
            );
            // Fallback color in case of unexpected creation error (unlikely here)
            "#808080".to_string() // Gray
        }
    }
}
