# Rust

## Examples

* `rough_fps` - A very dirty demonstration of manipulating rendering registers to display a blinking FPS counter.
* `rtd_id` - Renders the Cyclone V chip ID and the current, updating Unix time on the screen via Slint.
* `vblank` - Watches [the control registers](/docs/control.md) to determine when vblank starts and ends, and when vsync itself occurs.

## Overview

The `riscv32imafdc` Rust target doesn't exist as something you can install via `rustup`, and for some reason the `rustc` args to add additional architecture extensions don't work, so instead we use a custom build target. Luckily Cargo lets you set a local target via a JSON file; this can be found as `riscv32imafdc-unknown-none-elf.json` in each of the Rust platform directories (unfortunately I can't seem to make it work from a single, central location). We use the base `riscv32imac` toolchain that is officially supported, and add in the FPU operations (`fd`).

This custom target is specified in `/.cargo/config.toml`, at the root of the workspace (and repo). This file sets up our custom JSON target, adds additional linker arguments, enables building `core` and `alloc` core crates for this new target (requires nightly) and provides additional envvars to fix compilation with `cc` for any C/C++ dependencies.

Finally, your parent level Rust crate requires a `build.rs` script to add a linker search path to `/lang/linker/` (TODO: Can this be set in the `/.cargo/config.toml` file instead?):

```rust
let dest_path = Path::new("../../../linker/");

// I have no idea why this must be canonical and can't use relative paths
println!(
    "cargo:rustc-link-search={}",
    dest_path.canonicalize().unwrap().display()
);
```

**NOTE:** You must have Rust nightly installed and use it for all of your build operations. The `no_std` core libraries won't build without nightly.

## Setup

```bash
cd lang/rust/examples/rtc_id

# Install Rust nightly
rustup install nightly

# (Optional) Set nightly as default
# Alternatively, you could set a workspace level override to nightly (or just make all of your commands nightly)
rustup default nightly

# Install the base toolchain
rustup target add riscv32imac-unknown-none-elf

# Set up tools to build the final elf binary (cargo objcopy)
cargo +nightly install cargo-binutils
rustup +nightly component add llvm-tools-preview
```

## Design

Two crates are provided for an improved Rust coding experience:

* `litex-pac` - The Rust peripheral access crate (PAC) to the custom LiteX core. This provides strongly typed access to all of the registers and constants in the SoC's CSR segment.
* `litex-openfpga` - A set of useful primitives for interacting with the SoC. Provides definitions for `println!()`, filesystem access IO, and an optional `slint` feature for a Slint UI platform.

## Building

As stated above, you need to ensure you have the `/.cargo/config.toml` and `riscv32imafdc-unknown-none-elf.json` files in your workspace directory, and the linker path update code in `build.rs`.

If you have updated your LiteX platform, register locations and other constants may have changed, and thus you need to rebuild the `litex-pac` crate. You will want to execute `make` in `/lang/rust/crates/litex-pac`. This converts the `/litex/pocket.svd` file into Rust code to access the registers in a safe manner.

Unless you are building for another platform, there is no reason to produce debug builds (they will massively bloat your binary size). Thus builds look like:

```bash
cargo build --release
cargo objcopy --release -- -O binary ../../rust.bin
```

The `cargo objcopy` command builds a flat elf binary that can actually be run on the target plaform.