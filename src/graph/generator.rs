use crate::core::{params::Params, state::State};
use crate::memory::matrix::MemoryMatrix;

pub struct GraphGenerator {
    _params: Params,
}

impl GraphGenerator {
    pub fn new(params: Params) -> Self {
        Self { _params: params }
    }

    pub fn parents(&self, index: usize, state: &State, memory: &MemoryMatrix) -> Vec<Vec<u8>> {
        let mut parents_data = Vec::new();

        // 1. Sequential Dependency: Always include the immediately preceding block
        if index > 0 {
            parents_data.push(memory.get(index - 1).to_vec());
        }

        // 2. Dynamic Dependency: Use the current state to pick a random previous block
        if index > 1 {
            let state_bytes = state.as_bytes();
            
            // Convert the first 4 bytes of the state into a determinisic u32 integer
            let random_val = u32::from_le_bytes([
                state_bytes[0], state_bytes[1], state_bytes[2], state_bytes[3]
            ]) as usize;

            // Use modulo math to ensure the index falls within the blocks we've already computed
            let dynamic_parent_index = random_val % index;
            parents_data.push(memory.get(dynamic_parent_index).to_vec());
        }

        // If it's the very first block (index 0), it just returns an empty list, 
        // and the Mixer will just hash the initial State.
        parents_data
    }
}