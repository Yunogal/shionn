pub struct VSD {
    pub signature: [u8; 4], // 'SVD1'
    pub offset: u32,
}

pub fn extract(content: &[u8]) -> &[u8] {
    let offset = unsafe { content.as_ptr().add(4).cast::<u32>().read() };
    let address = offset + 8;
    &content[address as usize..]
}

#[test]
#[ignore]
fn main() -> std::io::Result<()> {
    use memmap2::MmapOptions;
    use std::fs::File;
    let file = File::open(r".svd")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let _ = extract(&mut mmap[..]);
    Ok(())
}
