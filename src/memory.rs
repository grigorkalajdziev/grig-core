use sha3::{Digest, Sha3_512};

pub const BLOCK_SIZE: usize = 1024;

#[derive(Clone)]
pub struct MemoryBlock {
    pub data: [u8; BLOCK_SIZE],
}

pub fn allocate_memory(size: usize) -> Vec<MemoryBlock> {
    vec![
        MemoryBlock {
            data: [0u8; BLOCK_SIZE],
        };
        size
    ]
}

pub fn initialize_memory(
    memory: &mut [MemoryBlock],
    state: &crate::state::GrigState,
) {

    for (i, block) in memory.iter_mut().enumerate() {

        let mut hasher = Sha3_512::new();

        hasher.update(&state.bytes);
        hasher.update(i.to_le_bytes());

        let hash = hasher.finalize();

        for j in 0..BLOCK_SIZE {
            block.data[j] = hash[j % 64];
        }
    }
}