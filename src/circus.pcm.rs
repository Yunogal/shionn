use std::fs::File;
use std::io::{Read, Result, Write};
use std::ptr;

use flate2::read::ZlibDecoder;

#[repr(C)]
pub struct PCM0 {
    pub signature: [u8; 4], // 'XPCM'
    pub zsize: u32,
    pub type_: u8,
    pub unknown: u8,
    pub unknown2: u16,
    //https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformat
    pub format_tag: u16,
    pub channal: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
}

#[repr(C)]
pub struct PCM3 {
    pub signature: [u8; 4], // 'XPCM'
    pub zsize: u32,
    pub type_: u8,
    pub unknown: u8,
    pub unknown2: u16,
    //https://learn.microsoft.com/en-us/windows/win32/api/mmreg/ns-mmreg-waveformat
    pub format_tag: u16,
    pub channal: u16,
    pub samples_per_sec: u32,
    pub avg_bytes_per_sec: u32,
    pub block_align: u16,
    pub bits_per_sample: u16,
    pub size: u32,
}

#[repr(C)]
pub struct PCM5 {
    pub signature: [u8; 4], // 'XPCM'
    pub zsize: u32,
    pub type_: u8,
    pub unknown: u8,
    pub unknown2: u16,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<PCM0>(), 4);
    assert_eq!(size_of::<PCM0>(), 28);
    assert_eq!(align_of::<PCM3>(), 4);
    assert_eq!(size_of::<PCM3>(), 32);
    assert_eq!(align_of::<PCM5>(), 4);
    assert_eq!(size_of::<PCM5>(), 16);
}
pub fn extract(content: &[u8]) -> Result<()> {
    let ptr: *const PCM5 = content.as_ptr().cast();
    let PCM5 { zsize, type_, .. } = unsafe { ptr.read() };
    match type_ {
        | 5 => {
            let mut file = File::create(".ogg")?;
            file.write_all(&content[16..])?;
        },
        | 0 => {
            let mut file = File::create(".pcm")?;
            file.write_all(&content[28..])?;
        },
        | 3 => {
            let mut data = Box::<[u8]>::new_uninit_slice(zsize as usize);
            let mut decode = ZlibDecoder::new(&content[32..]);
            let slice = unsafe {
                &mut *ptr::slice_from_raw_parts_mut(
                    data.as_mut_ptr() as *mut u8,
                    zsize as usize,
                )
            };
            let _ = unsafe { data.assume_init() };
            decode.read_exact(slice)?;
            let mut file = File::create(".pcm")?;
            file.write_all(slice)?;
        },
        | _ => {},
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    let file = File::open(r".pcm")?;
    let mmap = unsafe { MmapOptions::new().map(&file)? };
    extract(&mmap[..])?;
    Ok(())
}
