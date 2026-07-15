use sha3::{Digest, Sha3_512};

use crate::memory::MemoryBlock;
use crate::state::GrigState;

pub struct Node {
    pub index: usize,
    pub p1: usize,
    pub p2: usize,
    pub p3: usize,
}

pub fn generate_node(
    index: usize,
    state: &GrigState,
    memory: &[MemoryBlock],
) -> Node {

    let size = memory.len();

    let p1 = if index == 0 { size - 1 } else { index - 1 };

    let p2 = hash_to_index(state, index, size);
    let p3 = hash_to_index_memory(state, &memory[index], size);

    Node {
        index,
        p1,
        p2,
        p3,
    }
}

fn hash_to_index(
    state: &GrigState,
    i: usize,
    size: usize,
) -> usize {

    let mut hasher = Sha3_512::new();

    hasher.update(&state.bytes);
    hasher.update(i.to_le_bytes());

    let result = hasher.finalize();

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&result[..8]);

    usize::from_le_bytes(bytes) % size
}

fn hash_to_index_memory(
    state: &GrigState,
    block: &MemoryBlock,
    size: usize,
) -> usize {

    let mut hasher = Sha3_512::new();

    hasher.update(&state.bytes);
    hasher.update(&block.data);

    let result = hasher.finalize();

    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&result[..8]);

    usize::from_le_bytes(bytes) % size
}