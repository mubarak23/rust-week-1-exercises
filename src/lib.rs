use std::io::Read;
use std::num::ParseIntError;

#[derive(Debug)]
enum Errors {
    IoError(std::io::Error),
    HexError(ParseIntError),
    TooShortError,
}

impl From<std::io::Error> for Errors {
    fn from(err: std::io::Error) -> Self {
        Errors::IoError(err)
    }
}

impl From<ParseIntError> for Errors {
    fn from(err: ParseIntError) -> Self {
        Errors::HexError(err)
    }
}

fn read_version_byte(transaction_bytes: &mut &[u8]) -> Result<u32, Errors> {
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
    // let  stream_bytes = &transaction_bytes;
    let mut stream_bytes: &[u8] = &transaction_bytes;
    match read_version_byte(&mut stream_bytes) {
        Ok(version) => Ok(version),
        Err(Errors::IoError(e)) => Err(format!("I/O error reading version: {}", e)),
        Err(Errors::TooShortError) => Err("Transaction data too short".to_string()),
        Err(Errors::HexError(_)) => Err("Unexpected hex error during version read".to_string()),
    }
    // todo!()
}
