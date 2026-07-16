use blake2::{Blake2b512, Digest};

pub fn mix(inputs: &[&[u8]], state: &[u8], index: usize) -> Vec<u8> {
    let mut hasher = Blake2b512::new();

    for input in inputs {
        hasher.update(input);
    }

    hasher.update(state);
    hasher.update(index.to_le_bytes());

    hasher.finalize().to_vec()
}