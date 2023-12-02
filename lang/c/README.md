# C/C++

## Examples

* `helloworld` - A simple hello world demonstration in C, with the included `printf()` function.
* `helloworld-cpp` - A simple hello world class demonstration in C++, with the included `printf()` function.
* `fungus` - A demo with visuals, sound and controls.

## Overview

C/C++ is one of the simplest targets because you just need the appropriate `riscv[32/64]-unknown-elf-gcc` and supporting tools, otherwise known as the RISC-V GNU Toolchain. LiteX generates/includes a number of supporting libraries with utility methods and constants for accessing the various parts of the SoC. If you use the included `Makefile` (see any of the `/example` directories) and set up the variables at the top of the file, you will automatically pull in all of those object libraries.

This functionality is provided thanks to the generated `variables.mak`, containing variables and references to various directories, and the LiteX included `common.mak` file, which combines the set variables to create the constructs for the user's `Makefile`. A shared linker assembly, which is shared with other languages such as Rust, is located at `/lang/linker`.

## Setup

[Build and install the RISC-V GNU Toolchain](https://github.com/riscv/riscv-gnu-toolchain):

(Taken from [the base README](/README.md))

**NOTE:** The Ubuntu repository version of the toolchain is missing some functionality. You may need to manually compile the toolchain anyway.

```bash
cd ~

# Clone the repo
git clone https://github.com/riscv/riscv-gnu-toolchain.git

# Install dependencies
...

# Build the newlib, multilib variant of the toolchain
./configure --prefix=/opt/riscv --enable-multilib
make
```

This will produce binaries like `riscv64-unknown-elf-gcc`. Note that even though they're named `riscv64`, they can be used to build for `riscv32`. The `--enable-multilib` allows building for various RISC-V extensions, so we don't have to create a specialized version of the toolchain.

## Design

LiteX unfortunately does not provide documentation for the libraries that are automatically included; you can see them at `/litex/vendor/litex/litex/soc/software/`. The libraries are:

* `libbase`
* `libc`
* `libcompiler_rt`
* `libfatfs` - Probably not useful
* `liblitedram` - I wouldn't recommend directly controlling SDRAM, but you can if you wish
* `libliteeth` - Probably not useful
* `liblitesata` - Probably not useful
* `liblitesdcard` - Probably not useful
* `liblitespi`

There is a collection of custom functions and constants, generated off of SoC parameters such as configuration (CSR) registers, clock speed, etc., available at `/litex/build/litex/software/include/generated/`. This should prevent the need to use a SVD parser like Rust does.

## Building

Due to the libraries and headers LiteX provides, you need to do a full project build in order to have the right assets ready to go.

```bash
cd litex

make
```

See [README](/README.md#modifying-the-hardware) for more instructions on how to get the required dependencies and set up the build.

----

The provided C/C++ `Makefile` should set everything up for you. Update the variables at the top of the file, making sure to include all of your object names, and simply run `make`. When it completes, you should find a `build.bin` in your output directory; this is your RISC-V program ready to be copied over or [uploaded to the CPU over UART](/README.md#uart).
