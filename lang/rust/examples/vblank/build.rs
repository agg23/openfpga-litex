use std::path::Path;

/// Put the linker script somewhere the linker can find it.
fn main() {
    let dest_path = Path::new("../../../linker/");

    // I have no idea why this must be canonical and can't use relative paths
    println!(
        "cargo:rustc-link-search={}",
        dest_path.canonicalize().unwrap().display()
    );

    println!("cargo:rerun-if-changed=regions.ld");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
