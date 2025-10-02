use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

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

use std::os::raw::c_char;
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn extract(content: &[u8]) -> Result<()> {
    let ptr = content.as_ptr();
    let count = unsafe { ptr.cast::<u32>().read() };
    let entrys = unsafe {
        &*ptr::slice_from_raw_parts(ptr.add(4).cast::<Entry>(), count as usize)
    };
    let mut ptr = unsafe { ptr.add(4 + 12 * count as usize) };
    for entry in entrys {
        let address = entry.address as usize;
        let size = entry.size as usize;
        let name: &str = unsafe {
            let len = strlen(ptr.cast());
            let name = &*ptr::slice_from_raw_parts(ptr, len);
            ptr = ptr.add(len + 1);
            transmute(name)
        };
        let data = &content[address..address + size];
        let mut file = File::create(name)?;
        file.write_all(data)?;
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    let file = File::open(r".pck")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
