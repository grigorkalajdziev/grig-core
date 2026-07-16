use sha3::{Digest, Sha3_512};

pub struct State {
    internal: Vec<u8>,
}

impl State {
    pub fn initialize(password: &[u8], salt: &[u8], _params: &super::params::Params) -> Self {
        let mut hasher = Sha3_512::new();
        hasher.update(password);
        hasher.update(salt);
        
        Self { 
            internal: hasher.finalize().to_vec() 
        }
    }

    pub fn update(&mut self, memory: &crate::memory::matrix::MemoryMatrix, index: usize) {
        let mut hasher = Sha3_512::new();
        hasher.update(&self.internal);
        hasher.update(memory.get(index));
        
        self.internal = hasher.finalize().to_vec();
    }

    // NEW: Allow other modules to read the state without consuming it
    pub fn as_bytes(&self) -> &[u8] {
        &self.internal
    }

    pub fn finalize(self) -> Vec<u8> {
        self.internal
    }
}