use std::fs::{File, create_dir_all};
use std::io;
use std::path::Path;

use memmap2::Mmap;

use shionn::pfs;

fn main() -> io::Result<()> {
    let path = Path::new(".shionn");

    create_dir_all(path)?;

    let file = File::open(Path::new("example.pfs"))?;

    let mmap = unsafe { Mmap::map(&file)? };

    let _ = pfs::extract(mmap, Path::new(".shionn"));

    Ok(())
}
