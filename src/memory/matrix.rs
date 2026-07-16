pub struct MemoryMatrix {
    blocks: Vec<Vec<u8>>,
}

impl MemoryMatrix {
    pub fn new(size: usize, block_size: usize) -> Self {
        Self {
            blocks: vec![vec![0u8; block_size]; size],
        }
    }

    pub fn get(&self, i: usize) -> &[u8] {
        &self.blocks[i]
    }

    pub fn set(&mut self, i: usize, data: Vec<u8>) {
        self.blocks[i] = data;
    }
}