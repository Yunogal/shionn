#![allow(unnecessary_transmutes)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::manual_rotate)]
#![allow(clippy::needless_range_loop)]
#![allow(non_snake_case)]

use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use memmap2::MmapOptions;

#[path = "amuse.pac.rs"]
mod amuse_pac;
#[path = "artemis.pfs.rs"]
mod artemis_pfs;
#[path = "bgi.arc.rs"]
pub mod bgi_arc;
#[path = "bgi.dsc.rs"]
mod bgi_dsc;
mod exe;
mod kirikiri;
#[path = "kirikiri.xp3.rs"]
pub mod kirikiri_xp3;
#[path = "nexas.pac.rs"]
mod nexas_pac;
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
        let mut mmap = unsafe { MmapOptions::new().map_copy(&file)? };
        let base = &shionn.output.unwrap_or(PathBuf::from(".shionn"));
        fs::create_dir_all(base);
        let content = &mut mmap[..];
        match content {
            | [b'P', b'A', b'C', b'\x20', ..] => {
                //PAC\x20
                amuse_pac::extract(content, base);
            },
            | [b'P', b'A', b'C', ..] => {
                //PAC
                nexas_pac::extract(content, base);
            },
            | [b'p', b'f', ..] => {
                //pf8 //pf6
                artemis_pfs::extract(content, base);
            },
            | [b'Y', b'P', b'F', b'\0', ..] => {
                //YPF\0
                //ypf::extract(mmap, base);
            },
            // | [
            //     b'B',
            //     b'U',
            //     b'R',
            //     b'I',
            //     b'K',
            //     b'O',
            //     b'\x20',
            //     b'A',
            //     b'R',
            //     b'C',
            //     b'2',
            //     b'0',
            //     ..,
            // ] => {
            //     //BURIKO ARC20
            //     bgi_arc::extract(content, base)?;
            // },
            | [
                b'X',
                b'P',
                b'3',
                // b'\x0d',
                // b'\x0a',
                // b'\x20',
                // b'\x0a',
                // b'\x1a',
                // b'\x8b',
                // b'\x67',
                // b'\x01',
                ..,
            ] => {
                //xp3::extract(mmap, base)?;
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
