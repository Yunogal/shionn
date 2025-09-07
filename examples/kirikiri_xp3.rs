use std::fs::File;
use std::io;
use std::path::Path;

use memmap2::{MmapMut, MmapOptions};

use shionn::kirikiri::*;
use shionn::kirikiri_xp3;

fn main() -> io::Result<()> {
    let file = File::open(Path::new("example.xp3"))?;
    let mut mmap: MmapMut = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    let base = Path::new(".shionn");
    let game = Box::new(null);
    kirikiri_xp3::general(content, base, game)?;
    Ok(())
}
