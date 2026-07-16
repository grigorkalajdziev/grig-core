use grig_core::core::params::Params;

fn grig_hash(password: &[u8], salt: &[u8], params: Params) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(password);
    buf.extend_from_slice(salt);
    buf.push((params.blocks & 0xff) as u8);
    buf.push((params.rounds & 0xff) as u8);
    buf.push((params.parents & 0xff) as u8);

    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut hasher = DefaultHasher::new();
    buf.hash(&mut hasher);
    let h = hasher.finish();
    h.to_be_bytes().to_vec()
}

fn main() {
    let password = b"password123";
    let salt = b"salty";

    let params = Params {
        blocks: 16,
        rounds: 2,
        parents: 0,
    };

    let hash = grig_hash(password, salt, params);

    println!("Grig hash: {:x?}", hash);
}