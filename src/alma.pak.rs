use std::fs::File;
use std::io::Write;
use std::mem::transmute;
use std::ptr;

use crate::shionn_lzss;

#[repr(C)]
pub struct Pak {
    pub signature: [u8; 16],  // 'DataPack5\0\0\0\0\0\0\0'
    pub signature2: [u8; 32], //'EDEN'
    pub version: u32,
    pub size: u32,
    pub flags: u32,
    pub count: u32,
    pub start: u32,
    pub address: u32,
}

#[repr(C)]
pub struct Entry {
    pub name: [u8; 0x40],
    pub offset: u32,
    pub size: u32,
    pub unk: [u32; 8],
}

impl Entry {
    pub fn name(&self) -> &[u8] {
        let mut len: usize = 0;
        for i in self.name {
            if i != 0x00 {
                len += 1;
            }
        }
        &self.name[..len]
    }
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Pak>(), 4);
    assert_eq!(size_of::<Pak>(), 0x48);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 0x68);
}

pub fn extract(content: &mut [u8]) {
    let ptr: *const Pak = content.as_ptr().cast();
    let &Pak {
        size,
        flags,
        count,
        start,
        address,
        ..
    } = unsafe { &*ptr };
    let count = count as usize;
    let address = address as usize;
    let start = start as usize;
    let ptr = ptr::addr_of_mut!(content[address]);
    let zsize = count * 0x68;
    let slice = unsafe { &mut *ptr::slice_from_raw_parts_mut(ptr, size as usize) };
    if flags & 1 != 0 {
        for (index, i) in slice.iter_mut().enumerate() {
            *i ^= index as u8;
        }
    }
    let mut entry = Box::<[Entry]>::new_uninit_slice(count);
    let output = unsafe {
        &mut *ptr::slice_from_raw_parts_mut(entry.as_mut_ptr() as *mut u8, zsize)
    };
    shionn_lzss::lz(slice, output);
    let entry = unsafe { entry.assume_init() };

    let data = &mut content[start..];
    for i in 0..count {
        let name = entry[i].name();
        let mut key = 0;
        for &letter in name {
            key = key * 0x25 + (letter as u32 | 0x20)
        }
        let offset = entry[i].offset as usize;
        let ptr = ptr::addr_of_mut!(data[offset]);
        let size = entry[i].size as usize;
        let origin = unsafe { &*ptr::slice_from_raw_parts_mut(ptr, size) };
        let slice = unsafe {
            &mut *ptr::slice_from_raw_parts_mut(ptr as *mut u32, size >> 2)
        };
        for i in slice {
            *i ^= key;
        }
        let name: &str = unsafe { transmute(name) };
        let mut file = File::create(name).unwrap();
        file.write_all(origin).unwrap();
    }
}
