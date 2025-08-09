use std::fs;
use std::path::Path;

use shionn::ypf;

fn main() {
    let path = Path::new(".shionn");
    let _ = fs::create_dir_all(path);

    let _ = ypf::extract(Path::new("example.ypf"), path);
}
