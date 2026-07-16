pub struct MemoryMatrix {
    blocks: Vec<Vec<u8>>,
}

impl MemoryMatrix {
    pub fn new(size: usize) -> Self {
        Self {
            blocks: vec![vec![0u8; 64]; size],
        }
    }

    pub fn get(&self, index: usize) -> &[u8] {
        &self.blocks[index]
    }

    pub fn set(&mut self, index: usize, data: Vec<u8>) {
        self.blocks[index] = data;
    }
}