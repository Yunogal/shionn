use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::path::Path;
use std::ptr;

#[repr(C, align(4))]
pub struct Arc {
    signature: [u8; 12],
    count: u32,
}

#[repr(C, align(4))]
pub struct Entry {
    pub name: [u8; 0x60],
    pub address: u32,
    pub size: u32,
    pub useless: [u8; 0x18],
}
impl Entry {
    pub fn name(&self) -> &str {
        let mut len: usize = 0;
        for i in self.name {
            if i != 0x00 {
                len += 1;
            }
        }
        unsafe { transmute(&self.name[..len]) }
    }
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Arc>(), 4);
    assert_eq!(size_of::<Arc>(), 0x10);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 0x80);
}

pub fn extract(content: &[u8], base: &Path) -> Result<()> {
    let count = unsafe {
        let ptr = content.as_ptr().add(12) as *const u32;
        *ptr as usize
    };
    let end = 16 + count * 0x80;
    let entry = &content[16..end];
    let entry = entry.as_ptr() as *const Entry;

    let entry = unsafe { &*ptr::slice_from_raw_parts(entry, count) };

    for i in 0..count {
        let name = entry[i].name();
        let address = entry[i].address as usize + end;
        let size = entry[i].size as usize;
        let mut file = File::create(base.join(name))?;
        let content = &content[address..address + size];
        match &content[..16] {
            | b"DSC FORMAT 1.00\0" => {
                //let a = parse(content)?;
                file.write_all(&content)?;
            },
            | b"CompressedBG___\0" => {
                // let mut check: Box<[MaybeUninit<u8>]> =
                //     Box::new_uninit_slice(declength);
                // let mut sum: u8 = 0;
                // let mut xor = 0;
                // for i in 0..declength {
                //     let temp = check_[i] - update(&mut key);
                //     check[i] = MaybeUninit::new(temp);
                //     sum = sum + temp;
                //     xor ^= temp;
                // }
                // if sum != checksum && xor != checkxor {
                //     return Ok(());
                // }
                // let _check = unsafe { check.assume_init() };
                // return Ok(());
            },
            | _ => {},
        }
    }
    Ok(())
}
