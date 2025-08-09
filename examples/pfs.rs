use std::path::Path;

use shionn::pfs;

fn main() {
    let _ = pfs::extract(Path::new("example.pfs"), Path::new(".shionn"));
}
