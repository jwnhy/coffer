use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=link-qemu-64.ld");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    fs::File::create(out_dir.join("link-virtio-64.ld"))
        .unwrap()
        .write_all(include_bytes!("link-qemu-64.ld"))
        .unwrap();

    println!("cargo:rustc-link-search={}", out_dir.display());
}
