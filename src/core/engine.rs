use crate::{
    core::{params::Params, state::State},
    crypto::mixer::Mixer,
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
        let mut state = State::initialize(password, salt, &self.params);
        let mut memory = MemoryMatrix::new(self.params.blocks);

        let graph = GraphGenerator::new(self.params.clone());
        let mixer = Mixer::new();

        for _round in 0..self.params.rounds {
            for i in 0..self.params.blocks {
                let parents = graph.parents(i, &state, &memory);
                let block = mixer.mix(&parents, &state);

                memory.set(i, block);
                state.update(&memory, i);
            }
        }

        state.finalize()
    }
}