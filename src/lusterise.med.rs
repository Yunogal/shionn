use core::ptr;
use std::fs::File;
use std::io::{Result, Write};

use std::mem::transmute;

#[repr(C)]
pub struct Med {
    pub signature: [u8; 2], // 'MD'
    pub unk: u16,
    pub len: u16,
    pub count: u16,
    pub unk2: [u8; 8],
}

// pub struct Entry {
//     pub name: [u8; len - 4 - 4],
//     pub size: u32,
//     pub address: u32,
// }

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Med>(), 2);
    assert_eq!(size_of::<Med>(), 16);
}

pub fn extract(content: &[u8]) -> Result<()> {
    let mut ptr = content.as_ptr();
    let data_ptr = ptr;
    let med: *const Med = ptr.cast();
    let Med { len, count, .. } = unsafe { med.read() };
    let name_len = len as usize - 4 - 4;
    unsafe { ptr = ptr.add(16) };
    for _ in 0..count {
        unsafe {
            let name = &*ptr::slice_from_raw_parts(ptr, name_len - 1);
            let name: &str = transmute(name);
            ptr = ptr.add(name_len);
            let size = ptr.cast::<u32>().read_unaligned();
            let address = ptr.cast::<u32>().add(1).read_unaligned();
            ptr = ptr.add(8);
            let data = &*ptr::slice_from_raw_parts(
                data_ptr.add(address as usize),
                size as usize,
            );
            let mut file = File::create(name)?;
            file.write_all(data)?;
        }
    }
    Ok(())
}

#[test]
pub fn main() -> Result<()> {
    use memmap2::Mmap;
    let file =
        File::open(r"F:\GALGAME\NEXTON\Lusterise\光翼戦姫エクスティア\md_bgm.med")?;
    let mmap = unsafe { Mmap::map(&file)? };
    let content = &mmap[..];
    let _ = extract(content)?;
    Ok(())
}
