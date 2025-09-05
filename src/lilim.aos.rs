use std::ptr;

#[repr(C, packed)]
pub struct Aos {
    pub zero: u32,
    pub address: u32,
    pub size: u32,
    pub name: [u8; 0x105],
}

#[derive(Debug)]
#[repr(C)]
pub struct Entry {
    pub name: [u8; 32],
    pub offset: u32,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Aos>(), 1);
    assert_eq!(size_of::<Aos>(), 0x111);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 40);
}
pub fn extract(content: &[u8]) {
    let ptr: *const Aos = content.as_ptr().cast();
    let aos = unsafe { &*ptr };
    let count = aos.size / 40;
    let len = count as usize;
    let ptr: *const Entry = ptr::addr_of!(content[0x111]).cast();
    let entry = unsafe { &*ptr::slice_from_raw_parts(ptr, len) };

    let content = &content[aos.address as usize..];
    for i in entry {
        let start = i.offset as usize;
        let end = start + i.size as usize;
        let _data = &content[start..end];
    }
}
