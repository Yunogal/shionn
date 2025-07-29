use std::path::{Path, PathBuf};

use clap::{CommandFactory, Parser};

mod arc;
mod pac;

#[derive(Parser)]
#[command(version, about = "extract resource files", long_about = None)]
struct Shionn {
    /// input file
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    file: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let shionn = Shionn::parse();

    if let Some(path) = shionn.input.or(shionn.file).as_deref() {
        if let Some(ext) = path.extension() {
            if ext == "pac" {
                let _ = pac::extract(path, Path::new("shionn"));
            } else if ext == "arc" {
                let _ = arc::extract(path, Path::new("shionn"));
            } else {
                println!("(•_•)");
            }
        } else {
            println!("(•_•)");
        }
    } else {
        Shionn::command().print_help().unwrap();
        std::process::exit(0);
    }

    match shionn.debug {
        _ => {}
    }
}
