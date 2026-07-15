use sha3::{Digest, Sha3_512};

use crate::graph::Node;
use crate::memory::MemoryBlock;
use crate::state::GrigState;

pub fn mix_node(
    node: &Node,
    memory: &mut [MemoryBlock],
    state: &GrigState,
) {

    let mut hasher = Sha3_512::new();

    hasher.update(&memory[node.p1].data);
    hasher.update(&memory[node.p2].data);
    hasher.update(&memory[node.p3].data);
    hasher.update(&state.bytes);

    let result = hasher.finalize();

    for i in 0..memory[node.index].data.len() {
        memory[node.index].data[i] = result[i % 64];
    }
}