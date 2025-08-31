use std::ptr;

use crate::yuka_ykg;
use crate::yuka_yks;

#[derive(Debug)]
#[repr(C)]
pub struct YKC {
    pub signature: [u8; 6], //'YKC001'
    pub enctyption: u16,
    pub head_size: u32,
    pub zero: u32,
    pub address: u32,
    pub size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Entry {
    pub name_addr: u32,
    pub name_len: u32,
    pub address: u32,
    pub size: u32,
    pub zero: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<YKC>(), 4);
    assert_eq!(size_of::<YKC>(), 24);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 20);
}

pub fn exact(content: &mut [u8]) {
    let ptr: *const YKC = content.as_ptr().cast();
    let ykc = unsafe { &*ptr };
    let count = ykc.size / 20;
    let ptr: *const Entry = ptr::addr_of!(content[ykc.address as usize]).cast();
    let entry = unsafe { &*ptr::slice_from_raw_parts(ptr, count as usize) };
    for i in entry {
        let start = i.name_addr as usize;
        let end = i.name_addr as usize + i.name_len as usize - 1;
        let _name = &content[start..end];
        let start = i.address as usize;
        let end = i.address as usize + i.size as usize;
        let data = &mut content[start..end];
        if &data[..6] == b"YKC001" {
            yuka_yks::parse(data);
        } else if &data[..6] == b"YKG000" {
            let _data = yuka_ykg::parse(data);
        }
        break;
    }
}
