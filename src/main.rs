#![allow(unnecessary_transmutes)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(clippy::write_with_newline)]
#![allow(clippy::manual_rotate)]
#![allow(clippy::needless_range_loop)]

use std::fs;
use std::path::PathBuf;

use clap::{CommandFactory, Parser};
use regex::Regex;

mod arc;
mod arc_bgi;
mod exe;
mod pac;
mod pfs;
mod pna;

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

fn main() {
    let shionn = Shionn::parse();

    if let Some(path) = shionn.input.or(shionn.file).as_deref() {
        let name = path.file_name().unwrap_or_default();
        if let Some(file_str) = name.to_str() {
            let re = Regex::new(r"^.+\.(?P<ext>[^.]+)(?:\.\d+)*$").unwrap();
            if let Some(caps) = re.captures(file_str) {
                let output = shionn.output.unwrap_or(PathBuf::from(".shionn"));
                fs::create_dir_all(&output);
                match &caps["ext"] {
                    | "ws2" => {
                        if shionn.extra.is_none() {
                            println!("Requires additional parameters (such as *.exe)");
                        } else {
                            exe::check(&shionn.extra.unwrap(), path);
                        }
                    },
                    | "pfs" => {
                        pfs::extract(path, &output);
                    },
                    | "pac" => {
                        pac::extract(path, &output);
                    },
                    | "pna" => {
                        pna::extract(path, &output);
                    },
                    | "arc" => {
                        arc::extract(path, &output, shionn.sub_extract.unwrap_or(true));
                    },
                    | _ => {
                        println!("Are you sure this file is supported?(•_•)");
                    },
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
