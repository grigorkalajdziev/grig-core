use crate::api::hash::grig_hash;

#[test]
fn test_deterministic_output() {
    let password = b"password";
    let salt = b"salt";

    let h1 = grig_hash(password, salt);
    let h2 = grig_hash(password, salt);

    assert_eq!(h1, h2);
}

#[test]
fn test_known_vector() {
    let password = b"password";
    let salt = b"salt";

    let hash = grig_hash(password, salt);

    println!("{:x?}", hash); // copy this output once

    let expected = vec![
        0x7f, 0x6b, 0x03, 0x3f, 0x53, 0xe9, 0x85, 0x68,
        0x09, 0xa4, 0xa8, 0x45, 0xa3, 0x44, 0x93, 0x1f,
        0xee, 0x8b, 0xdb, 0xe3, 0x01, 0x34, 0xd7, 0x99,
        0x06, 0xe4, 0xa7, 0x9c, 0xf3, 0x64, 0x84, 0x7e,
        0x6c, 0x77, 0x33, 0xf0, 0xdf, 0xae, 0x8b, 0x08,
        0x7b, 0xb9, 0x8a, 0x7c, 0x8e, 0x75, 0x1e, 0x3b,
        0x5e, 0x8b, 0xd6, 0xd6, 0x89, 0x34, 0x15, 0x57,
        0xe4, 0xd9, 0xea, 0xf2, 0xe7, 0xa1, 0x18, 0x51,
    ];

    assert_eq!(hash, expected);
}

#[test]
fn test_avalanche_effect() {
    let salt = b"salt";

    let h1 = grig_hash(b"password", salt);
    let h2 = grig_hash(b"password1", salt);

    assert_ne!(h1, h2);
}

#[test]
fn test_salt_changes_output() {
    let password = b"password";

    let h1 = grig_hash(password, b"salt1");
    let h2 = grig_hash(password, b"salt2");

    assert_ne!(h1, h2);
}

#[test]
fn test_output_length() {
    let hash = grig_hash(b"password", b"salt");

    assert_eq!(hash.len(), 64); // BLAKE2b = 64 bytes
}

use crate::core::{engine::Engine, params::Params};

#[test]
fn test_param_changes_output() {
    let p1 = Params { blocks: 512, rounds: 2, parents: 4 };
    let p2 = Params { blocks: 4096, rounds: 6, parents: 6 };

    let e1 = Engine::new(p1);
    let e2 = Engine::new(p2);

    let h1 = e1.run(b"password", b"salt");
    let h2 = e2.run(b"password", b"salt");

    assert_ne!(h1, h2);
}

#[test]
fn test_large_memory_runs() {
    let params = Params {
        blocks: 4096,
        rounds: 6,
        parents: 6,
    };

    let engine = Engine::new(params);

    let hash = engine.run(b"password", b"salt");

    assert_eq!(hash.len(), 64);
}

use crate::security::constant_time::secure_eq;

#[test]
fn test_constant_time_eq() {
    let a = b"same_data";
    let b = b"same_data";
    let c = b"different";

    assert!(secure_eq(a, b));
    assert!(!secure_eq(a, c));
}

#[test]
fn test_empty_inputs() {
    let hash = grig_hash(b"", b"");
    assert_eq!(hash.len(), 64);
}

#[test]
fn test_long_inputs() {
    let password = vec![0u8; 10_000];
    let salt = vec![1u8; 1024];

    let hash = grig_hash(&password, &salt);

    assert_eq!(hash.len(), 64);
}