use memmap2::MmapOptions;
use shionn::anim_dat;
use std::io;
use std::{fs::File, io::Write};
fn main() -> io::Result<()> {
    let file = File::open("name.dat")?;
    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    anim_dat::parce(content);
    let mut file = File::create("name")?;
    file.write_all(content)?;
    Ok(())
}
