use std::fs::File;
use std::io::{Result, Write};
use std::ptr;

#[repr(C)]
pub struct PNA {
    pub signature: [u8; 4], // 'PNAP'
    pub unknown: u32,
    pub width: u32,
    pub height: u32,
    pub count: u32,
}

#[repr(C)]
pub struct Entry {
    pub unknown1: u32,
    pub index: u32,
    pub offset_x: u32,
    pub offset_y: u32,
    pub width: u32,
    pub height: u32,
    pub unknown2: u32,
    pub unknown3: u32,
    pub unknown4: u32,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<PNA>(), 4);
    assert_eq!(size_of::<PNA>(), 20);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 40);
}

pub fn parse(content: &[u8]) -> Result<()> {
    let ptr: *const PNA = content.as_ptr().cast();
    let PNA { count, .. } = unsafe { ptr.read() };
    let count = count as usize;
    let ptr = unsafe { ptr.add(1).cast::<Entry>() };
    let entry = unsafe { &*ptr::slice_from_raw_parts(ptr, count) };
    let mut ptr: *const u8 = unsafe { ptr.add(count) }.cast();
    let mut i = 0;
    for &Entry {
        // index,
        // offset_x,
        // offset_y,
        // width,
        // height,
        size,
        ..
    } in entry
    {
        let size = size as usize;
        if size == 0 {
            print!("1 ");
        }
        let buf = unsafe { &*ptr::slice_from_raw_parts(ptr, size) };
        let name = format!("{i}.png"); //
        let mut file = File::create(name)?;
        file.write_all(buf)?;
        ptr = unsafe { ptr.add(size) };
        i += 1;
    }
    Ok(())
}
#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r"example.pna")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    parse(&mut mmap[..])?;
    Ok(())
}
