pub struct YKG {
    pub signature: [u8; 6], //'YKG000'
    pub enctyption: u16,
    pub head_size: u32,
    pub zero: u32,
    pub unk: [u32; 6],
    pub offset: u32,
    pub size: u32,
    pub unk2: [u32; 2],
    pub offset2: u32,
    pub size2: u32,
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<YKG>(), 4);
    assert_eq!(size_of::<YKG>(), 0x40);
}

pub fn parse(content: &[u8]) -> &[u8] {
    let ptr: *const YKG = content.as_ptr().cast();
    let ykg = unsafe { &*ptr };
    let start = ykg.offset as usize;
    let end = ykg.offset as usize + ykg.size as usize;
    &content[start..end]
}
