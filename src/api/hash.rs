use crate::core::{engine::Engine, params::Params};

pub fn grig_hash(password: &[u8], salt: &[u8]) -> Vec<u8> {
    let params = Params {
        blocks: 4096,
        rounds: 6,
        parents: 6,
    };

    let engine = Engine::new(params);
    engine.run(password, salt)
}