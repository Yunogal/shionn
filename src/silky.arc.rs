use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

use crate::shionn_lzss;

pub struct Arc {
    pub count: u32,
}

// pub struct Entry {
//     pub len: u8,
//     pub name: [u8; len],
//     pub size: u32,
//     pub zsize: u32,
//     pub address: u32,
// }

pub fn extract(content: &mut [u8]) -> Result<()> {
    let mut ptr = content.as_mut_ptr();
    let buf_ptr = ptr;
    let head_size = unsafe { ptr.cast::<u32>().read() };
    unsafe { ptr = ptr.add(4) }
    let end_ptr = unsafe { ptr.add(head_size as usize) };
    let mut output: Vec<u8> = Vec::with_capacity(4 * 1024 * 1024);
    unsafe {
        while ptr < end_ptr {
            let len = ptr.read();
            let mut key = len;
            let name = &mut *ptr::slice_from_raw_parts_mut(ptr.add(1), len as usize);
            for i in name.iter_mut() {
                *i += key;
                key -= 1;
            }
            let name: &str = transmute(name);

            ptr = ptr.add(len as usize + 1);
            let u32_ptr: *const u32 = ptr.cast();
            let size = u32_ptr.read_unaligned().swap_bytes() as usize;
            let zsize = u32_ptr.add(1).read_unaligned().swap_bytes() as usize;
            let address = u32_ptr.add(2).read_unaligned().swap_bytes() as usize;
            ptr = ptr.add(12);
            let buf = &*ptr::slice_from_raw_parts(buf_ptr.add(address), size);
            let mut file = File::create(name)?;
            if zsize == size {
                file.write_all(buf)?;
            } else {
                output.set_len(zsize);
                shionn_lzss::lz(buf, &mut output);
                file.write_all(&output)?;
            }
        }
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r".arc")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
