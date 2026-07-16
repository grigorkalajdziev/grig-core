use crate::core::{engine::Engine, params::Params};

pub fn grig_hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    let params = Params {
        blocks: 1024,
        rounds: 3,
        parents: 4,
    };

    let engine = Engine::new(params);
    engine.run(password, salt)
}