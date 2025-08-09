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
        ".shionn/data.a",
        "example.pac",
        "example.pfs.0001",
        "a.",
        "1.a2",
        "1  1.example",
    ];
    for f in &examples {
        println!("{f} => {:?}", extract_ext(f));
    }
}
