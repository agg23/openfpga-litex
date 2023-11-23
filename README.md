# Custom LiteX SoC for Analogue Pocket

This SoC is intended to serve as a host platform for any kind of software that may be useful/fun/interesting to run on the Analogue Pocket, such as calculators, cart dumpers, internet access, custom game consoles/emulation, and more. The system is constructed to provide access to as many of the Pocket's core systems as possible in a simple, software-friendly manner. As opportunities arrive, some functionality may be hardware accelerated.

The core is not intended for use by end users directly, but for developers, who may use either the distributed built assets (the `agg23.RISCV....zip` release) or this repo itself, customized to provide the experience desired.

## Features

* Full input handing
* ~~Cart slot and link port access~~ - `Coming soon`
* Serial bus, with program hot reloading, via USB Blaster JTAG adapter or Analogue Pocket Dev Kit UART cart
* File access API for load/store - [See Analogue Docs](https://www.analogue.co/developer/docs/core-definition-files/data-json) - `Writing to disk coming soon`
* Vblank and vsync access; frame counter
* 48kHz audio playback with 4096 sample buffer
* Live RTC access
* Cyclone V unique chip ID access

## Hardware Definition

General RISC-V notation for this core is: `riscv32imafdc`. See [Standard Extensions](https://en.wikichip.org/wiki/risc-v/standard_extensions) to learn what each of the letters mean.

Notably, this core contains:

* 32 bit CPU, buses, RAM
* Atomics
* A FPU supporting both 32 and 64 bit floats

Pocket RAM used:

* 31% of FPGA BRAM
* SDRAM

## General Operation

### Data

The core automatically loads and boots from the `data.json` slot id 0 entry. This defaults to a file named `/Assets/riscv/common/boot.bin`. Replacing this allows you to automatically start your own program. The program is written to `0x4000_0000`, with the jump vector at that address.

Adding additional dataslots allows you to request reads/writes via [File API](./docs/control.md#file-api). You can choose the address you send the data to. I would recommend not clobbering your loaded program, but maybe you can do something neat with self-modifying programs.

### UART

The core supports UART over serial and JTAG, and can boot programs over UART. JTAG requires [a supported USB Blaster](https://www.digikey.com/en/products/detail/terasic-inc/P0302/2003484) (**NOTE:** Don't buy a Chinese clone. This has killed at least one Pocket). Serial requires the developer kit dev cart (not available for purchase), OR you could build the cart yourself (contact me and our group of devs will help you figure out what you need). `/lang/rust` gives an example of how to write to the UART with `println!()`; just like writing to `stdout`.

The serial UART is pinned to the max speed supported by the dev cart, 2,000,000bps. Serial UART is disabled when JTAG is enabled. You can connect using the LiteX tooling via:
```bash
python3 ./litex/vendor/litex/litex/tools/litex_term.py --speed 2000000 /dev/ttyUSB0

# To automatically upload a program named `rust.bin`
python3 ./litex/vendor/litex/litex/tools/litex_term.py --speed 2000000 --kernel rust.bin /dev/ttyUSB0
```

JTAG UART must be explicitly enabled in the [Core Settings](#core-settings). JTAG UART has fewer options:
```bash
python3 ./litex/vendor/litex/litex/tools/litex_term.py --jtag-config=openocd_usb_blaster.cfg jtag

# To automatically upload a program named `rust.bin`
python3 ./litex/vendor/litex/litex/tools/litex_term.py --jtag-config=openocd_usb_blaster.cfg --kernel rust.bin jtag
```

The kernel program will be uploaded on core reset, so you can either start the core fresh, or reset it from the menu (or configured reset button).

You may opt to send a custom command over UART to your running core that causes reset. This would allow for full automation and deployment of a program when building.

### Core Settings

Core Settings are defined in `interact.json`, and the core defaults with two options:

* `Enable + Btn Reset` - When enabled, the + button (to the right of the Analogue button) will be configured to act as a core reset. This is handy if you are actively developing a core, and quickly want to upload a new version.
* `Enable JTAG UART` - When enabled, the JTAG adapter will be used for platform UART; the serial UART port will be disabled.

When releasing your core, you likely want to remove both of these options, as they are not likely to be beneficial to your users.

## Getting Started

### Software Only

You can simply [download the latest release](https://github.com/agg23/openfpga-litex2/releases/latest) and start writing code. For language setup help and examples, see [Languages](/lang/README.md).

I suggest you make this repo a submodule of your main project so you can reference import libraries and files, along with pinning the core version.

### Customize Hardware

**NOTE:** You will probably have a bad time if you try to do this with Windows. It should be possible, but I just found lots of pain.

```bash
# Enter your working directory
mkdir riscv
cd riscv

# Clone repo with submodules
git clone --recursive https://github.com/agg23/openfpga-litex.git

# Create a Python virtualenv with your manager of choice
...

# Install Python dependencies
pip3 install pyserial
```

[Build and install the RISC-V GNU Toolchain](https://github.com/riscv/riscv-gnu-toolchain):

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

This will produce binaries like `riscv64-unknown-elf-gcc`. Note that even though they're `riscv64`, they can be used to build for `riscv32`. The `--enable-multilib` allows building for various RISC-V extensions, so we don't have to create a specialized version of the toolchain.

To build the LiteX SoC:

```bash
cd openfpga-litex/litex/
make
```

You can now build the project via Quartus, either through the UI or via CLI.

## Writing Software

[Guides and examples for several languages are available](/lang/README.md). Check those first.

The `/lang/linker/` directory contains the required linker files for you to build against the SoC. `memory.x` was written by hand, and `regions.ld` is generated by the LiteX build process and copied over to this directory.

You need to build against the [specific RISC-V extension target used by the core](#hardware-definition). Using the standard `riscv32-unknown-elf-gcc`, building for this architecture would look something like this:

```bash
riscv32-unknown-elf-gcc -march=rv32imacfd -mabi=ilp32fd
```

Notably, you _must_ build targetting the FPU (`-mabi=ilp32fd`). Software built targetting a soft-float implementation will not run.

### Accessing Control Registers

To actually do anything with the hardware, you need to access many different control registers. These provide the CPU with an interface to modify how the hardware SoC operates. LiteX provides this nice, generic mechanism for specifying control information via [the SVD file](/litex/pocket.svd). This file provides a machine readable list of all the interaction points in the SoC and provides short descriptions on how to use them. [A human readable description, with additional details, is also provided](/docs/control.md).

The SVD file can be used to generate support packages to automatically keep your program address constants up to date with the current iteration of the hardware. An example of this can be found in the [Rust `litex-pac` crate](/lang/rust/crates/litex-pac).

## Modifying the Hardware

* [Adjusting Resolution](/docs/resolution.md)