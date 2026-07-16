use crate::crypto::hash;

pub struct State {
    value: Vec<u8>,
}

impl State {
    pub fn new(password: &[u8], salt: &[u8]) -> Self {
        let mut init = Vec::new();
        init.extend_from_slice(password);
        init.extend_from_slice(salt);

        Self {
            value: hash::hash(&init),
        }
    }

    pub fn update(&mut self, block: &[u8], index: usize) {
        let mut data = Vec::new();
        data.extend_from_slice(&self.value);
        data.extend_from_slice(block);
        data.extend_from_slice(&index.to_le_bytes());

        self.value = hash::hash(&data);
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.value
    }
}