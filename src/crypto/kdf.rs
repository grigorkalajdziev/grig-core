use hkdf::Hkdf;
use sha2::Sha256;

pub fn derive_key(input: &[u8], salt: &[u8], info: &[u8]) -> Vec<u8> {
    let hk = Hkdf::<Sha256>::new(Some(salt), input);
    let mut okm = [0u8; 32];

    hk.expand(info, &mut okm).unwrap();

    okm.to_vec()
}