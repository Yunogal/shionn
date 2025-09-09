use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

#[repr(C)]
pub struct SCS {
    pub signature: [u8; 8], // 'SZS120__'
    pub unknown: u32,
    pub count: u32,
}

#[repr(C)]
pub struct Entry {
    pub name: [u8; 0x100],
    pub address: u64,
    pub size: u64,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<SCS>(), 4);
    assert_eq!(size_of::<SCS>(), 16);
    assert_eq!(align_of::<Entry>(), 8);
    assert_eq!(size_of::<Entry>(), 0x110);
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

pub fn extract(content: &mut [u8]) -> Result<()> {
    let ptr: *const SCS = content.as_ptr().cast();
    let SCS { count, .. } = unsafe { ptr.read() };
    let entry = unsafe {
        &*ptr::slice_from_raw_parts(ptr.add(1).cast::<Entry>(), count as usize)
    };
    for i in entry {
        let name = i.name();
        let address = i.address as usize;
        let size = i.size as usize;
        println!("{name}");
        let data = unsafe {
            &mut *ptr::slice_from_raw_parts_mut(
                ptr::addr_of_mut!(content[address]),
                size,
            )
        };
        for i in data.iter_mut() {
            *i ^= 0x90;
        }
        let mut file = File::create(name)?;
        file.write_all(data)?;
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r".szs")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
