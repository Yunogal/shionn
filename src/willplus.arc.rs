use std::{fs::File, io::Result};
use std::{io::Write, ptr};

use encoding_rs::UTF_16LE;

#[repr(C)]
pub struct Arc {
    pub count: u32,
    pub size: u32,
}

// pub struct Entry {
//     pub size: u32,
//     pub offset: u32,
//     pub name:[u8;unknown]
// }

unsafe extern "system" {
    fn lstrlenW(s: *const u16) -> usize;
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Arc>(), 4);
    assert_eq!(size_of::<Arc>(), 8);
}
pub fn extract(content: &mut [u8]) -> Result<()> {
    let ptr: *const Arc = content.as_ptr().cast();
    let Arc { count, size } = unsafe { ptr.read() };
    let count = count as usize;
    let size = size as usize;
    let mut ptr: *const u16 = unsafe { ptr.add(1) }.cast();
    let mut temp: *const u32 = ptr.cast();
    let data = &mut content[size + 8..];
    for _ in 0..count {
        unsafe {
            let size = temp.read_unaligned() as usize;
            let offset = temp.add(1).read_unaligned() as usize;
            ptr = temp.add(2).cast();
            let len = lstrlenW(ptr);
            let bytes = &*ptr::slice_from_raw_parts(ptr.cast::<u8>(), 2 * len);
            let (name, ..) = UTF_16LE.decode(bytes);
            ptr = ptr.add(len + 1);
            temp = ptr.cast();
            let data = &mut data[offset..offset + size];
            if &name[name.len() - 4..] == ".ws2" {
                for i in data.iter_mut() {
                    *i = (*i >> 2) | (*i << 6);
                }
            }
            let mut file = File::create(name.as_ref())?;
            file.write(data)?;
        }
    }
    Ok(())
}

#[test]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r"example.arc")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
