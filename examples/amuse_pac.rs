use std::fs::{File, create_dir_all};
use std::io;
use std::path::Path;

use memmap2::MmapOptions;

use shionn::amuse_pac;

fn main() -> io::Result<()> {
    //Folders where you want to store
    let path = Path::new(".shionn");

    create_dir_all(path)?;

    //Replace `example.pac` with the file you actually need to use
    let file = File::open(Path::new("example.pac"))?;

    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    amuse_pac::extract(content, path)?;

    Ok(())
}
