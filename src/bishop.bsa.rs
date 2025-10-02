use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

#[repr(C)]
pub struct BSA {
    pub signature: [u8; 8], // 'BSArc\0\0\0'
    pub version: u16,
    pub count: u16,
    pub address: u32,
}

#[repr(C)]
pub struct Entry {
    pub name_offset: u32,
    pub address: u32,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<BSA>(), 4);
    assert_eq!(size_of::<BSA>(), 16);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 12);
}

use std::os::raw::c_char;
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}
pub fn extract(content: &mut [u8]) -> Result<()> {
    let ptr = content.as_ptr();
    let BSA { count, address, .. } = unsafe { ptr.cast::<BSA>().read() };
    let ptr = unsafe { ptr.add(address as usize) };
    let entry = unsafe {
        let ptr: *const Entry = ptr.cast();
        &*ptr::slice_from_raw_parts(ptr, count as usize)
    };
    let ptr = unsafe { ptr.add(0x0C * count as usize) };
    for i in entry {
        let &Entry {
            name_offset,
            address,
            size,
        } = i;
        let name: &str = unsafe {
            let data = ptr.add(name_offset as usize);
            let len = strlen(ptr.cast());
            let name = &*ptr::slice_from_raw_parts(data, len);
            transmute(name)
        };
        let mut file = File::create(name)?;
        file.write_all(
            &content[address as usize..address as usize + size as usize],
        )?;
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    let file = File::open(r".bsa")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
