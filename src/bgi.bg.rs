#[repr(C)]
pub struct CompressedBG {
    signature: [u8; 16],
    width: u16,
    height: u16,
    bbp: u32,
    size: u32,
    key: u32,
    zsize: u32,
    checksum: u32,
    checkxor: u32,
    version: u32,
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<CompressedBG>(), 4);
    assert_eq!(size_of::<CompressedBG>(), 48);
}
