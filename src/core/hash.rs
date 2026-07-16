use super::{engine::Engine, params::Params};

pub fn grig_hash(password: &[u8], salt: &[u8], params: Params) -> Vec<u8> {
    let engine = Engine::new(params);
    engine.run(password, salt)
}