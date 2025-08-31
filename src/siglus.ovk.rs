use std::ptr;

#[repr(C)]
pub struct Ovk {
    pub count: u32,
}

#[repr(C)]
pub struct Entry {
    pub size: u32,
    pub address: u32,
    pub id: u32,
    pub unk: u32,
}
#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Ovk>(), 4);
    assert_eq!(size_of::<Ovk>(), 4);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 16);
}

pub fn extract(content: &[u8]) {
    let count: u32 = unsafe { *content.as_ptr().cast() };
    let ptr: *const Entry = unsafe { content.as_ptr().add(4).cast() };
    let entrys = unsafe { &*ptr::slice_from_raw_parts(ptr, count as usize) };
    for entry in entrys {
        let _address = entry.address as usize;
        let _size = entry.size as usize;
    }
}
