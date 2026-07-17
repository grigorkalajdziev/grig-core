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

    let expected_hex = "7dc0e98c2e0cfaceb04fd388279f3e2f0ba5a84996cccfeae6c77b87cf379a671c8dedcbe60c70b14ad23531a380a713f783f61af22f486adb32c92872bf9a68";
    let expected = hex::decode(expected_hex).unwrap();

    println!("{:x?}", hash); 

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