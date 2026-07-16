use blake2::{Blake2b512, Digest};

pub fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}