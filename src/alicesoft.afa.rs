use std::fs::File;
use std::io::{Read, Result, Write};
use std::ptr;

use encoding_rs::SHIFT_JIS;
use flate2::read::ZlibDecoder;

use crate::shionn_stream::ByteStream;

pub struct Afa {
    pub signature: [u8; 4], // 'AFAH'
    pub len: u32,
    pub signature2: [u8; 8], // 'AlicArch'
    pub version: u32,
    pub unk: u32,
    pub start_offset: u32,
    pub signature3: [u8; 4], // 'INFO'
    pub size: u32,
    pub zsize: u32,
    pub count: u32,
}

// pub struct Entry {
//     name_size: u32,
//     buf_size: u32,
//     name: [u8; buf_size],
//     unk: u32,
//     unk2: u32,
// if Afa::version==1 {unk3:u32}else{unk3:u0}
//     offset: u32,
//     size: u32,
// }

const KEY: [u8; 16] = [
    0xC8, 0xBB, 0x8F, 0xB7, 0xED, 0x43, 0x99, 0x4A, 0xA2, 0x7E, 0x5B, 0xB0, 0x68,
    0x18, 0xF8, 0x88,
];

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<Afa>(), 4);
    assert_eq!(size_of::<Afa>(), 0x2C);
}

pub fn extract(content: &mut [u8]) -> Result<()> {
    let ptr: *const Afa = content.as_ptr().cast();
    let &Afa {
        version,
        start_offset,
        size,
        zsize,
        count,
        ..
    } = unsafe { &*ptr };
    let size = size as usize;
    let zsize = zsize as usize;
    let count = count as usize;
    let mut output = Box::<[u8]>::new_uninit_slice(zsize);
    let buf = unsafe {
        &mut *ptr::slice_from_raw_parts_mut(output.as_mut_ptr() as *mut u8, zsize)
    };
    let mut decode = ZlibDecoder::new(&content[0x2C..0x2C + size]);
    decode.read_exact(buf)?;
    let output = unsafe { output.assume_init() };
    let mut stream = ByteStream::new(&output);
    let content = &mut content[start_offset as usize..];
    let skip_ = if version < 2 { 12 } else { 8 };
    for _ in 0..count {
        let name_size = stream.read::<u32>() as usize;
        let buf_size = stream.read::<u32>() as usize;
        let name = stream.get_no_ahead(name_size);
        stream.skip(buf_size + skip_);
        let offset = stream.read::<u32>() as usize;
        let size = stream.read::<u32>() as usize;
        let data = &mut content[offset..offset + size];
        if data[..3] == *b"AFF" {
            for (index, i) in data.iter_mut().enumerate() {
                *i ^= KEY[index & 15];
            }
        }
        let (name, ..) = SHIFT_JIS.decode(name);
        let mut file = File::create(name.as_ref())?;
        file.write_all(data)?;
    }
    Ok(())
}

#[test]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r"example")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
