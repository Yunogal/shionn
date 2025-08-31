#[repr(C)]
pub struct YKS {
    pub signature: [u8; 6],
    pub enctyption: u16,
    pub head_size: u32,
    pub zero: u32,
    pub instr_offset: u32,
    pub instr_count: u32,
    pub index_offset: u32,
    pub index_count: u32,
    pub offset: u32,
    pub size: u32,
    pub max: u32,
    pub zero2: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<YKS>(), 4);
    assert_eq!(size_of::<YKS>(), 48);
}

pub fn parse(content: &mut [u8]) {
    let ptr: *const YKS = content.as_ptr().cast();
    let ykc = unsafe { &*ptr };
    let start = ykc.offset as usize;
    for i in &mut content[start..] {
        *i ^= 0xAA;
    }
}
