use sha3::{Digest, Keccak256};

pub fn hash_passwd(password_text: String)  -> String { 
    let mut hasher = Keccak256::new();
    hasher.update(password_text.as_bytes());
    hex::encode(hasher.finalize())
}