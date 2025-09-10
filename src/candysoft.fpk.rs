use std::fs::File;
use std::io::{Result, Write};
use std::mem::transmute;
use std::ptr;

use crate::shionn_readwrite::{ReadStream, WriteStream};

#[repr(transparent)]
pub struct FPK {
    pub count: u32,
}
//30 26 B2 75 8E 66 CF 11

#[repr(C)]
pub struct Entry {
    pub address: u32,
    pub size: u32,
    pub name: [u8; 0x18],
    pub serial: u32,
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

#[repr(C)]
pub struct ZLC2 {
    pub signature: [u8; 4], // 'ZLC2'
    pub zsize: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<FPK>(), 4);
    assert_eq!(size_of::<FPK>(), 4);
    assert_eq!(align_of::<Entry>(), 4);
    assert_eq!(size_of::<Entry>(), 36);
    assert_eq!(align_of::<ZLC2>(), 4);
    assert_eq!(size_of::<ZLC2>(), 8);
}

pub fn extract(content: &mut [u8]) -> Result<()> {
    // if content[..8] == *b"\x30\x26\xb2\x75\x8e\x66\xcf\x11" {
    //     // Advanced Systems Format
    //     // WMV
    //     return Ok(());
    // }
    let ptr = content.as_ptr();
    let mut count = unsafe { *ptr.cast::<u32>() };
    if count < 0x80000000 {} //
    count &= 0x7fffffff;
    let len = content.len();
    let key = unsafe {
        ptr.add(len - 8)
            .cast::<[u8; 4]>()
            .read_unaligned()
    };
    let address =
        unsafe { ptr.add(len - 4).cast::<u32>().read_unaligned() } as usize;
    let size = count as usize * size_of::<Entry>();
    let meta = &mut content[address..address + size];
    for (index, i) in meta.iter_mut().enumerate() {
        *i ^= key[index & 3];
    }
    let entry = unsafe {
        &*ptr::slice_from_raw_parts(meta.as_ptr().cast::<Entry>(), count as usize)
    };

    for i in entry {
        let address = i.address as usize;
        let size = i.size as usize;

        let name = i.name();
        let mut file = File::create(name)?;
        if content[address..address + 4] == *b"ZLC2" {
            let zsize = unsafe {
                ptr::addr_of!(content[address + 4])
                    .cast::<u32>()
                    .read_unaligned()
            } as usize;
            let mut output = Box::<[u8]>::new_uninit_slice(zsize);
            let slice = unsafe {
                &mut *ptr::slice_from_raw_parts_mut(
                    output.as_mut_ptr() as *mut u8,
                    zsize,
                )
            };
            let data = &content[address + 8..address + size];
            unpack(data, slice);
            let output = unsafe { output.assume_init() };
            file.write_all(&output)?;
        } else {
            file.write_all(&content[address..address + size])?;
        }
    }

    Ok(())
}
fn unpack(input: &[u8], output: &mut [u8]) {
    let len = input.len();
    let len2 = output.len();
    let mut read = ReadStream::new(input);
    let mut write = WriteStream::new(output);

    while read.pos < len && write.pos < len2 {
        let control: u8 = read.read_aligned();
        let mut mask = 0x80u8;
        loop {
            if mask == 0 || read.pos >= len || write.pos >= len2 {
                break;
            }
            if control & mask != 0 {
                let offset: u8 = read.read_aligned();
                let count: u8 = read.read_aligned();
                let mut offset =
                    (offset as usize) | (((count as usize) & 0xF0) << 4);
                let mut count = ((count as usize) & 0x0F) + 3;
                if offset == 0 {
                    offset = 4096;
                }
                if write.pos + count >= len2 {
                    count = len2 - write.pos;
                }
                write.copy_from_self(offset, count);
            } else {
                write.copy_from(1, &mut read);
            };
            mask >>= 1;
        }
    }
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r"F:\GALGAME\CandySoft\LoveCommu\data.fpk")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
