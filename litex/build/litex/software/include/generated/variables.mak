PACKAGES=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata bios
PACKAGE_DIRS=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libc /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libcompiler_rt /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libbase /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libfatfs /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitespi /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitedram /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libliteeth /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesdcard /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesata /mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/bios
LIBS=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata
TRIPLE=riscv64-unknown-elf
CPU=vexriscv
CPUFAMILY=riscv
CPUFLAGS= -march=rv32i2p0_mafdc -mabi=ilp32d -D__vexriscv__ -DUART_POLLING
CPUENDIANNESS=little
CLANG=0
CPU_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/cores/cpu/vexriscv_smp
SOC_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc
PICOLIBC_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/pythondata-software-picolibc/pythondata_software_picolibc/data
PICOLIBC_FORMAT=integer
COMPILER_RT_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/pythondata-software-compiler_rt/pythondata_software_compiler_rt/data
export BUILDINC_DIRECTORY
BUILDINC_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/build/litex/software/include
LIBC_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libc
LIBCOMPILER_RT_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libcompiler_rt
LIBBASE_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libbase
LIBFATFS_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libfatfs
LIBLITESPI_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitespi
LIBLITEDRAM_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitedram
LIBLITEETH_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/libliteeth
LIBLITESDCARD_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesdcard
LIBLITESATA_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/liblitesata
BIOS_DIRECTORY=/mnt/c/Users/adam/code/fpga/openfpga-litex/litex/vendor/litex/litex/soc/software/bios
LTO=0
BIOS_CONSOLE_FULL=1