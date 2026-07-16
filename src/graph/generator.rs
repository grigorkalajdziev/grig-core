use crate::crypto::hash::hash;

pub struct GraphGenerator {
    pub parents: usize,
}

impl GraphGenerator {
    pub fn select(&self, index: usize, state: &[u8], total: usize) -> Vec<usize> {
        let mut seed = Vec::new();
        seed.extend_from_slice(state);
        seed.extend_from_slice(&index.to_le_bytes());

        let h = hash(&seed);

        let mut result = Vec::new();

        for i in 0..self.parents {
            let mut bytes = [0u8; 8];
            bytes.copy_from_slice(&h[i * 8..(i + 1) * 8]);

            result.push((u64::from_le_bytes(bytes) as usize) % total);
        }

        // enforce sequential dependency
        if index > 0 {
            result.push(index - 1);
        }

        result
    }
}