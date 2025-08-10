#![allow(unnecessary_transmutes)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::manual_rotate)]
#![allow(clippy::needless_range_loop)]

use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use memmap2::MmapOptions;

mod arc;
mod arc_bgi;
mod exe;
mod pac_amuse;
mod pac_nexas;
mod pfs;
mod pna;
mod ypf;

#[derive(Parser)]
#[command(version, about = "extract resource files", long_about = None)]
struct Shionn {
    /// input file
    #[arg(short, long, value_name = "file")]
    input: Option<PathBuf>,

    /// input file
    file: Option<PathBuf>,

    /// enable/disable sub extract
    #[arg(short, long, value_name = "true/false", default_value = "true")]
    sub_extract: Option<bool>,

    #[arg(short, long, value_name = "directory", default_value = ".shionn")]
    output: Option<PathBuf>,

    #[arg(short, long, value_name = "file")]
    extra: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let shionn = Shionn::parse();
    if let Some(path) = shionn.input.or(shionn.file).as_deref() {
        let file = OpenOptions::new().read(true).open(path)?;
        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let base = &shionn.output.unwrap_or(PathBuf::from(".shionn"));
        fs::create_dir_all(base);
        match mmap[..] {
            | [b'P', b'A', b'C', b'\x20', ..] => {
                //PAC\x20
                pac_amuse::extract(mmap, base);
            },
            | [b'P', b'A', b'C', b'v', ..] => {
                //PACv
                pac_nexas::extract(mmap, base);
            },
            | [b'p', b'f', b'8', ..] => {
                //pf8
                pfs::extract(mmap, base);
            },
            | [b'Y', b'P', b'F', b'\0', ..] => {
                //YPF\0
                ypf::extract(mmap, base);
            },

            | _ => {
                println!("Are you sure this file is supported?(•_•)");
            },
        }
        Ok(())
    } else {
        Shionn::command().print_help().unwrap();
        std::process::exit(0);
    }
}
