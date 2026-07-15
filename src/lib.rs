pub mod state;
pub mod memory;
pub mod graph;
pub mod mixer;
pub mod hash;

use state::*;
use memory::*;
use graph::*;
use mixer::*;
use hash::*;

pub struct Params {
    pub blocks: usize,
    pub rounds: usize,
}

pub fn grig_hash(
    password: &[u8],
    salt: &[u8],
    params: &Params,
) -> [u8; 64] {

    // 1. Initialize state
    let mut state = initialize_state(password, salt);

    // 2. Allocate memory
    let mut memory = allocate_memory(params.blocks);

    // 3. Initialize memory
    initialize_memory(&mut memory, &state);

    // 4. Main rounds
    for _ in 0..params.rounds {
        for i in 0..memory.len() {

            let node = generate_node(i, &state, &memory);

            mix_node(&node, &mut memory, &state);
        }

        state = evolve_state(&state, &memory);
    }

    // 5. Final hash
    finalize(&state, &memory)
}