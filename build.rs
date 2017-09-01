use std::env;

fn main() {
    println!("cargo:rustc-env=LIBRARY_PATH={}", env::var("CARGO_MANIFEST_DIR").unwrap() + "/lib");
}