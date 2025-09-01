use std::fs::{File, create_dir_all};
use std::io;
use std::path::Path;

use memmap2::MmapOptions;

use shionn::artemis_pfs;

fn main() -> io::Result<()> {
    let path = Path::new(".shionn");

    create_dir_all(path)?;

    let file = File::open(Path::new("selectoblige.pfs"))?;

    let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
    let content = &mut mmap[..];
    let _ = artemis_pfs::extract(content, Path::new(".shionn"));

    Ok(())
}
