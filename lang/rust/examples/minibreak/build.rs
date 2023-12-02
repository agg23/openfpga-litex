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

    {
        // Print a human-readable warning if the screen size is surprising.
        use litex_pac::constants::{MAX_DISPLAY_WIDTH, MAX_DISPLAY_HEIGHT};
        const MAX_DISPLAY_WIDTH_EXPECTED: u32 = 266;
        const MAX_DISPLAY_HEIGHT_EXPECTED: u32 = 240;
        if MAX_DISPLAY_WIDTH != MAX_DISPLAY_WIDTH_EXPECTED
            || MAX_DISPLAY_HEIGHT != MAX_DISPLAY_HEIGHT_EXPECTED
        {
            println!("cargo:warning=This app was designed for a screen of {MAX_DISPLAY_WIDTH_EXPECTED}x{MAX_DISPLAY_HEIGHT_EXPECTED}. It is being compiled for a screen of {MAX_DISPLAY_WIDTH}x{MAX_DISPLAY_HEIGHT}. Check to make sure it looks good (and if not, edit the \"App Properties\" constants in main.rs), then edit this warning in build.rs.");
        }
    }
}
