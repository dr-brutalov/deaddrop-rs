use sha2::{Sha256, Digest};
use log::{info};
use hex::ToHex;

pub fn message_hash(message: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(message.as_bytes());

    let result_tmp = hasher.finalize();

    // let result = str::from_utf8(&result_tmp).unwrap_or_else(|error| println!("The hasher error is: {}", error));
    info!("A message hash has successfully been processed!");
    
    result_tmp.encode_hex::<String>()
    
}