use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

pub struct Dat {
    pub size: u32,
    pub size_: u32,
}

#[repr(C)]
pub struct Entry {
    pub name: [u8; 0x104],
    pub offset: u32,
    pub size: u32,
}

use std::os::raw::c_char;
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn extract(content: &[u8]) -> Result<()> {
    let mut ptr: *const u32 = content.as_ptr().cast();
    let size = unsafe {
        let size = *ptr;
        if size == 0 {
            ptr = ptr.add(1);
            *ptr
        } else {
            size
        }
    };
    let mut ptr: *const Entry = unsafe { ptr.add(1).cast() };

    let count = size / 0x10C;
    let buf_ptr = unsafe { ptr.add(count as usize) };
    let data: [u8; 0x104] = [0; 0x104]; //  0x10E-0x10E+0x104 system.dat
    let mut key = [0; 4];
    let len = unsafe { strlen(data.as_ptr().cast()) };
    for (index, i) in data[1..=len].iter().rev().enumerate() {
        key[index & 3] += *i;
    }
    for _ in 0..count {
        let mut _key = [0; 4];
        let len = unsafe { strlen(ptr.cast()) };
        let name = unsafe { &*ptr::slice_from_raw_parts(ptr.cast::<u8>(), len) };
        let name: &str = unsafe { transmute(name) };
        let data =
            unsafe { &*ptr::slice_from_raw_parts(ptr.cast::<u8>().add(1), len) };
        for (index, i) in data.iter().rev().enumerate() {
            _key[index & 3] += *i;
        }
        _key[0] += key[3];
        _key[1] += key[2];
        _key[2] += key[1];
        _key[3] += key[0];
        let key = u32::from_le_bytes(_key);
        let Entry { offset, size, .. } = unsafe { ptr.read() };
        let offset = offset ^ key;
        let size = size ^ key;
        let buf = unsafe {
            &*ptr::slice_from_raw_parts(
                buf_ptr.cast::<u8>().add(offset as usize),
                size as usize,
            )
        };
        let mut file = File::create(name)?;
        file.write_all(buf)?;
        unsafe { ptr = ptr.add(1) }
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r".dat")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
