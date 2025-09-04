use std::io::Read;

use flate2::read::ZlibDecoder;

#[repr(C)]
pub struct Hzc {
    pub signature: u32, //'hzc1'
    pub size: u32,
    pub header_size: u32,
    pub signature2: u32, //'NVSG'
    pub unk: u16,
    pub type_: u16,
    pub width: u16,
    pub height: u16,
    pub offset_x: u16,
    pub offset_y: u16,
    pub zero: u32,
    pub count: u32,
    pub zero2: u32,
    pub zero3: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Hzc>(), 4);
    assert_eq!(size_of::<Hzc>(), 0x2C);
}

pub fn parse(content: &[u8], output: &mut [u8]) {
    let ptr = content.as_ptr() as *const Hzc;
    let _hzc = unsafe { &*ptr };
    let mut decode = ZlibDecoder::new(&content[0x2C..]);
    decode.read_exact(output).unwrap();
}
