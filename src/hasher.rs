use sha2::{Sha256, Digest};
use hex::ToHex;

pub fn message_hash(message: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(message.as_bytes());

    let result_tmp = hasher.finalize();
    
    result_tmp.encode_hex::<String>()
    
}