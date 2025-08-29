use std::fs::{File, create_dir_all};
use std::io;
use std::path::Path;

use memmap2::Mmap;

use shionn::amuse_pac;

fn main() -> io::Result<()> {
    //Folders where you want to store
    let path = Path::new(".shionn");

    create_dir_all(path)?;

    //Replace `example.pac` with the file you actually need to use
    let file = File::open(Path::new("example.pac"))?;

    let mmap = unsafe { Mmap::map(&file)? };
    let content = &mmap[..];
    amuse_pac::extract(content, path)?;

    Ok(())
}
