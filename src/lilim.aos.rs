use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

#[repr(C, packed)]
pub struct Aos {
    pub zero: u32,
    pub address: u32,
    pub size: u32,
    pub name: [u8; 0x105],
}

#[repr(C)]
pub struct Entry {
    pub name: [u8; 32],
    pub offset: u32,
    pub size: u32,
}

use std::os::raw::c_char;
unsafe extern "C" {
    unsafe fn strlen(s: *const c_char) -> usize;
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Aos>(), 1);
    assert_eq!(size_of::<Aos>(), 0x111);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 40);
}
pub fn extract(content: &[u8]) -> Result<()> {
    let ptr: *const Aos = content.as_ptr().cast();
    let aos = unsafe { &*ptr };
    let count = aos.size / 40;
    let len = count as usize;
    let mut ptr: *const Entry = ptr::addr_of!(content[0x111]).cast();
    let content = &content[aos.address as usize..];
    for _ in 0..len {
        unsafe {
            let len = strlen(ptr.cast());
            let name = &*ptr::slice_from_raw_parts(ptr.cast::<u8>(), len);
            let name: &str = transmute(name);
            let Entry { offset, size, .. } = ptr.read_unaligned();
            let start = offset as usize;
            let end = start + size as usize;
            let data = &content[start..end];
            ptr = ptr.add(1);
            let mut file = File::create(name)?;
            file.write_all(data)?;
        }
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    let file = File::open(r"example.aos")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
