use clap::Parser;
use std::path::{Path, PathBuf};

mod pac;
use pac::extract;

#[derive(Parser)]
#[command(version, about = "extract resource files", long_about = None)]
struct Shionn {
    /// input file
    #[arg(short, long, value_name = "FILE")]
    input: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let shionn = Shionn::parse();

    if let Some(path) = shionn.input.as_deref() {
        let _ = extract(path, Path::new("shionn"));
    }

    match shionn.debug {
        _ => {}
    }
}
