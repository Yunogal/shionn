use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::ptr;

use encoding_rs::SHIFT_JIS;
use memmap2::Mmap;

#[derive(Debug)]
pub struct Entry {
    pub name_offset: u32,
    pub address: u32,
    pub size: u32,
}
pub struct Bin {
    pub count: u32,
    pub name_size: u32,
}

pub fn extract(mmap: Mmap, base: &Path) -> io::Result<()> {
    let content = &mmap[..];
    let ptr: *const Bin = content.as_ptr().cast();
    let bin = unsafe { &*ptr };
    let count = bin.count as usize;
    let _name_size = bin.name_size;
    let ptr: *mut Entry = unsafe { ptr.add(1) as _ };
    let entry = unsafe { &*ptr::slice_from_raw_parts_mut(ptr, count) };
    let pos = 8 + 12 * count as usize;
    let ext1 = entry[0].address as usize;
    let ext = match &content[ext1..ext1 + 3] {
        | b"Ogg" => ".ogg",
        | b"hzc" => ".hzc",
        | b"RIF" => ".wav",
        | _ => ".unknow",
    };
    let name_content = &content[pos..ext1];

    for i in 0..count {
        let Entry {
            name_offset,
            address,
            size,
        } = entry[i];
        let address = address as usize;
        let fin = address + size as usize;
        let start = name_offset as usize;
        let mut end = start;
        loop {
            if name_content[end] == 0 {
                break;
            }
            end += 1;
        }
        let (name, ..) = SHIFT_JIS.decode(&name_content[start..end]);
        let name = name + ext;
        let mut output_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(base.join(name.as_ref()))?;

        output_file.write_all(&content[address..fin])?;
    }

    Ok(())
}
