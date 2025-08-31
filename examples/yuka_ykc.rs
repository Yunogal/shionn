use memmap2::MmapOptions;
use std::fs::File;
use std::io;

use shionn::yuka_ykc;
fn main() -> io::Result<()> {
    let file = File::open("example.ykc")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    yuka_ykc::exact(content);
    Ok(())
}
