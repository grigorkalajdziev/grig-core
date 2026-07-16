use crate::core::state::State;
use sha3::{Digest, Sha3_512};

pub struct Mixer;

impl Mixer {
    pub fn new() -> Self {
        Self
    }

    pub fn mix(&self, inputs: &[Vec<u8>], state: &State) -> Vec<u8> {
        let mut hasher = Sha3_512::new();
        
        // FIX: Read the bytes safely without taking ownership of the State
        hasher.update(state.as_bytes());

        for input in inputs {
            hasher.update(input);
        }

        hasher.finalize().to_vec()
    }
}