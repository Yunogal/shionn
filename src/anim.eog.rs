pub struct Eog {
    pub signature: u32, //'CRM\0'
    pub unk: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Eog>(), 4);
    assert_eq!(size_of::<Eog>(), 8);
}

pub fn parse(content: &[u8]) -> &[u8] {
    &content[8..]
}
