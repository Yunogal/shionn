use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

#[repr(C)]
pub struct Bin {
    pub signature: [u8; 8], // 'ESC-ARC2'
    pub seed: Seed,
    pub count: u32,
    pub name_size: u32,
}

#[repr(C)]
pub struct Entry {
    pub name_offset: u32,
    pub address: u32,
    pub size: u32,
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Seed(u32);

impl Seed {
    #[inline]
    pub fn update(&mut self) -> u32 {
        self.0 ^= 0x65AC9365;
        self.0 ^= (((self.0 >> 1) ^ self.0) >> 3) ^ (((self.0 << 1) ^ self.0) << 3);
        self.0
    }
}

pub struct ACP {
    pub signature: [u8; 4], // 'acp\0'
    pub zsize: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Bin>(), 4);
    assert_eq!(size_of::<Bin>(), 20);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 12);
}

use std::os::raw::c_char;
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

pub fn extract(content: &mut [u8]) -> Result<()> {
    let ptr: *mut Bin = content.as_mut_ptr().cast();
    let &Bin {
        mut seed,
        mut count,
        name_size,
        ..
    } = unsafe { &*ptr };
    count ^= seed.update();
    let name_size = (name_size ^ seed.update()) as usize;
    let ptr = unsafe { ptr.add(1) }.cast::<u32>();
    let count = count as usize;
    let index_size = 3 * count;
    let index = unsafe { &mut *ptr::slice_from_raw_parts_mut(ptr, index_size) };
    for i in index.iter_mut() {
        *i ^= seed.update();
    }
    let entry = unsafe {
        &*ptr::slice_from_raw_parts(index.as_ptr().cast::<Entry>(), count as usize)
    };
    let name_ptr: *const i8 = unsafe { ptr.add(index_size) }.cast();
    unsafe {
        for i in 0..count {
            let Entry {
                name_offset,
                address,
                size,
            } = entry[i];
            let name_offset = name_offset as usize;
            let address = address as usize;
            let end = address + size as usize;
            let name_ptr = name_ptr.add(name_offset);
            let len = strlen(name_ptr);
            let name = &*ptr::slice_from_raw_parts(name_ptr.cast::<u8>(), len);
            let name: &str = transmute(name);
            let mut file = File::create(name)?;
            let buf = &content[address..end];
            if &buf[..4] == b"acp\0" {
                let zsize = ((buf[4] as usize) << 24)
                    + ((buf[5] as usize) << 16)
                    + ((buf[6] as usize) << 8)
                    + (buf[7] as usize);
                println!("{zsize}");
                //lzw;
            } else {
                file.write(buf)?;
            }
        }
    }
    Ok(())
}
#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    let file = File::open(r"example.bin")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    extract(content)?;
    Ok(())
}
