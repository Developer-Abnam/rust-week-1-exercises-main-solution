
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Check if the input is too short (needs at least 4 hex chars = 2 bytes for version)
    if raw_tx_hex.len() < 4 {
        return Err("Transaction data too short".to_string());
    }

    // Decode hex string to bytes
    let tx_bytes = hex::decode(raw_tx_hex).map_err(|e| format!("Hex decode error: {}", e))?;

    // Extract first 4 bytes (little-endian) for version
    let version_bytes = tx_bytes
        .get(0..4)
        .ok_or("Transaction data too short for version")?;

    // Convert bytes to u32 (little-endian)
    let version = u32::from_le_bytes([
        version_bytes[0],
        version_bytes[1],
        version_bytes[2],
        version_bytes[3],
    ]);

    Ok(version)
}