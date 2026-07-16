use crate::{
    core::{params::Params, state::State},
    crypto::mixer,
    graph::generator::GraphGenerator,
    memory::matrix::MemoryMatrix,
};

pub struct Engine {
    params: Params,
}

impl Engine {
    pub fn new(params: Params) -> Self {
        Self { params }
    }

    pub fn run(&self, password: &[u8], salt: &[u8]) -> Vec<u8> {
        let mut state = State::new(password, salt);
        let mut memory = MemoryMatrix::new(self.params.blocks, 64);

        let graph = GraphGenerator {
            parents: self.params.parents,
        };

        for _round in 0..self.params.rounds {
            for i in 0..self.params.blocks {
                let parent_idx = graph.select(i, state.as_bytes(), self.params.blocks);

                let inputs: Vec<&[u8]> =
                    parent_idx.iter().map(|&p| memory.get(p)).collect();

                let block = mixer::mix(&inputs, state.as_bytes(), i);

                memory.set(i, block.clone());
                state.update(&block, i);
            }
        }

        state.as_bytes().to_vec()
    }
}