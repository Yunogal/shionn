use std::fs;
use std::path::Path;

use shionn::pac;

fn main() {
    //Folders where you want to store
    let path = Path::new(".shionn");

    //Replace `example.pac` with the file you actually need to use
    let file = Path::new("example.pac");

    let _a = fs::create_dir_all(path);

    let _b = pac::extract(file, path);
}
