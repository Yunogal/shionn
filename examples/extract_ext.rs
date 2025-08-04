fn extract_ext(filename: &str) -> Option<&str> {
    filename
        .rsplit('.')
        .find(|s| !s.chars().all(|c| c.is_ascii_digit()))
}

fn main() {
    let examples = [
        "archive.tar.gz",
        "data.json.2",
        " ",
        ".a",
        "example.pac",
        "example.pfs.0001",
        "a.",
    ];
    for f in &examples {
        println!("{f} => {:?}", extract_ext(f));
    }
}
