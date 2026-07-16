use zeroize::Zeroize;

pub fn wipe(data: &mut [u8]) {
    data.zeroize();
}