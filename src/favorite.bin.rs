use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;
use std::ptr;

use encoding_rs::SHIFT_JIS;

#[repr(C)]
pub struct Bin {
    pub count: u32,
    pub name_size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Entry {
    pub name_offset: u32,
    pub address: u32,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Bin>(), 4);
    assert_eq!(size_of::<Bin>(), 8);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 12);
}

pub fn extract(content: &[u8], base: &Path) -> Result<()> {
    let ptr: *const Bin = content.as_ptr().cast();
    let &Bin { count, .. } = unsafe { &*ptr };
    let count = count as usize;
    let ptr: *const Entry = unsafe { ptr.add(1).cast() };
    let entry = unsafe { &*ptr::slice_from_raw_parts(ptr, count) };
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
        let mut file = File::create(base.join(name.as_ref()))?;
        file.write_all(&content[address..fin])?;
    }
    Ok(())
}
