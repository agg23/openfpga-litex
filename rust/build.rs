use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Put the linker script somewhere the linker can find it.
fn main() {
    let out_dir = env::var("OUT_DIR").expect("No out dir");
    let dest_path = Path::new(&out_dir);

    let mut f = File::create(&dest_path.join("memory.x")).expect("Could not create file");
    f.write_all(include_bytes!("memory.x"))
        .expect("Could not write file");

    let mut f = File::create(&dest_path.join("regions.ld")).expect("Could not create file");
    f.write_all(include_bytes!(concat!(
        env!("BUILD_DIR"),
        "/litex/software/include/generated/regions.ld"
    )))
    .expect("Could not write file");

    println!("cargo:rustc-link-search={}", dest_path.display());

    println!("cargo:rerun-if-changed=regions.ld");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");

    // slint_build::compile_with_config(
    //     "ui/main.slint",
    //     slint_build::CompilerConfiguration::new()
    //         .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    // )
    // .unwrap();

    // slint_build::compile_with_config(
    //     "ui/audio.slint",
    //     slint_build::CompilerConfiguration::new()
    //         .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    // )
    // .unwrap();

    slint_build::compile_with_config(
        "ui/rtc_id.slint",
        slint_build::CompilerConfiguration::new()
            .embed_resources(slint_build::EmbedResourcesKind::EmbedForSoftwareRenderer),
    )
    .unwrap();
}
