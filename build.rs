// https://doc.rust-lang.org/cargo/reference/build-script-examples.html

fn main() {
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    println!("cargo:warning= {:}", arch);
}
