use sha3::{Digest, Sha3_512};

use crate::memory::MemoryBlock;
use crate::state::GrigState;

pub fn finalize(
    state: &GrigState,
    memory: &[MemoryBlock],
) -> [u8; 64] {

    let mut hasher = Sha3_512::new();

    hasher.update(&state.bytes);
    hasher.update(&memory[0].data);
    hasher.update(&memory[memory.len() - 1].data);

    let result = hasher.finalize();

    let mut output = [0u8; 64];
    output.copy_from_slice(&result);

    output
}