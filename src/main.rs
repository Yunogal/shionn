#![allow(unnecessary_transmutes)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::manual_rotate)]
#![allow(clippy::needless_range_loop)]

use std::path::{Path, PathBuf};

use clap::{CommandFactory, Parser};
use regex::Regex;

mod arc;
mod pac;
mod pfs;

#[derive(Parser)]
#[command(version, about = "extract resource files", long_about = None)]
struct Shionn {
    /// input file
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    file: Option<PathBuf>,
}

fn main() {
    let shionn = Shionn::parse();

    if let Some(path) = shionn.input.or(shionn.file).as_deref() {
        let name = path.file_name().unwrap_or_default();
        if let Some(file_str) = name.to_str() {
            let re = Regex::new(r"^.+\.(?P<ext>[^.\d]+)(?:\.\d+)*$").unwrap();
            if let Some(caps) = re.captures(file_str) {
                match &caps["ext"] {
                    "pfs" => {
                        pfs::extract(path, Path::new("shionn"));
                    }
                    "pac" => {
                        pac::extract(path, Path::new("shionn"));
                    }
                    "arc" => {
                        arc::extract(path, Path::new("shionn"));
                    }
                    _ => {
                        println!("Are you sure this file is supported?(•_•)");
                    }
                }
            } else {
                println!("(•_•)");
            }
        }
    } else {
        Shionn::command().print_help().unwrap();
        std::process::exit(0);
    }
}
