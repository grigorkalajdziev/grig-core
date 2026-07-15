use grig_core::{grig_hash, Params};

fn main() {

    let password = b"password456";
    let salt = b"salty";

    let params = Params {
        blocks: 1024,   // ~1 MB
        rounds: 2,
    };

    let hash = grig_hash(password, salt, &params);

    println!("Grig hash: {:x?}", hash);
}