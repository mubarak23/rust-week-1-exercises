use std::io::{Error, Read};


fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Error> {
    let mut buffer = [0; 4];
    let _ = transaction_bytes.read_exact(&mut buffer); // read exactly 4 bytes from transaction_bytes into the buffer stream;
    Ok(u32::from_le_bytes(buffer)) // converts a 4-byte array (buffer) into a 32-bit unsigned integer (u32) assuming little-endian byte order
}

// Implement extract_tx_version function below
pub fn extract_tx_version(transaction_hex: &str) -> Result<u32, String> {
    let transaction_bytes = match hex::decode(transaction_hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Hex decode error".to_string()),
    };
    if transaction_bytes.len() < 4 {
        return Err("Transaction data too short".to_string());
    }
    
    let mut stream_bytes: &[u8] = &transaction_bytes;
    match read_version_byte(&mut stream_bytes) {
        Ok(version) => Ok(version),
        Err(e) => Err(format!("{}", e)),
    }
  
}
