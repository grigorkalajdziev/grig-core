use subtle::ConstantTimeEq;

pub fn secure_eq(a: &[u8], b: &[u8]) -> bool {
    a.ct_eq(b).into()
}