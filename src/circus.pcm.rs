use std::fs::File;
use std::io::{Result, Write};

pub struct PCM {
    pub signature: [u8; 4], // 'XPCM'
    pub zsize: u32,
    pub flag: u32,
    pub size: u32,
}

#[test]
fn size() {
    use std::mem::{align_of, size_of};
    assert_eq!(align_of::<PCM>(), 4);
    assert_eq!(size_of::<PCM>(), 16);
}
pub fn extract(content: &[u8]) -> Result<()> {
    let ptr: *const PCM = content.as_ptr().cast();
    let PCM { flag, .. } = unsafe { ptr.read() };
    if flag & 0xff == 0x05 {
        let mut file = File::create(".ogg")?;
        file.write_all(&content[16..])?;
    }
    Ok(())
}

#[test]
#[ignore]
fn main() -> Result<()> {
    use memmap2::MmapOptions;
    let file = File::open(r".pcm")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    extract(&mut mmap[..])?;
    Ok(())
}
