use sha3::{Digest, Sha3_512};

#[derive(Clone)]
pub struct GrigState {
    pub bytes: [u8; 64],
}

pub fn initialize_state(password: &[u8], salt: &[u8]) -> GrigState {
    let mut hasher = Sha3_512::new();
    hasher.update(password);
    hasher.update(salt);

    let result = hasher.finalize();

    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(&result);

    GrigState { bytes }
}

pub fn evolve_state(
    state: &GrigState,
    memory: &[crate::memory::MemoryBlock],
) -> GrigState {

    let mut hasher = Sha3_512::new();

    hasher.update(&state.bytes);
    hasher.update(&memory[0].data);
    hasher.update(&memory[memory.len() - 1].data);

    let result = hasher.finalize();

    let mut bytes = [0u8; 64];
    bytes.copy_from_slice(&result);

    GrigState { bytes }
}