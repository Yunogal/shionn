use memmap2::MmapOptions;
use std::fs::File;
use std::io::Result;

use shionn::alma_pak;
fn main() -> Result<()> {
    let file = File::open("example.pak")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    alma_pak::extract(content);
    Ok(())
}
