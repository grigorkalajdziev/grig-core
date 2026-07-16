use grig_core::core::{hash::grig_hash, params::Params};

fn main() {
    let password = b"password123";
    let salt = b"salty";

    let params = Params {
        blocks: 16,
        rounds: 2,
    };

    let hash = grig_hash(password, salt, params);

    println!("Grig hash: {:x?}", hash);
}