#!/usr/bin/env bash
set -e
# Create bin file
riscv64-unknown-elf-objcopy $1 -O binary $1.bin
# Program FPGA
cd ../vendor/
python3 run.py "litex/litex/tools/litex_term.py --kernel $1.bin /dev/ttyUSB0"