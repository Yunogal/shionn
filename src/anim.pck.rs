#[repr(transparent)]
pub struct Pck {
    pub count: u32,
}
#[repr(C)]
pub struct Entry {
    pub zero: u32,
    pub address: u32,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Pck>(), 4);
    assert_eq!(size_of::<Pck>(), 4);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 12);
}
