PACKAGES=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata bios
PACKAGE_DIRS=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libc /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libcompiler_rt /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libbase /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libfatfs /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitespi /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitedram /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libliteeth /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitesdcard /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitesata /home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/bios
LIBS=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata
TRIPLE=riscv64-unknown-elf
CPU=vexriscv
CPUFAMILY=riscv
CPUFLAGS= -march=rv32i2p0_mafdc -mabi=ilp32d -D__vexriscv__ -DUART_POLLING
CPUENDIANNESS=little
CLANG=0
CPU_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/cores/cpu/vexriscv_smp
SOC_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc
PICOLIBC_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/pythondata-software-picolibc/pythondata_software_picolibc/data
PICOLIBC_FORMAT=integer
COMPILER_RT_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/pythondata-software-compiler_rt/pythondata_software_compiler_rt/data
export BUILDINC_DIRECTORY
BUILDINC_DIRECTORY=/home/adam/openfpga-litex2/litex/build/litex/software/include
LIBC_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libc
LIBCOMPILER_RT_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libcompiler_rt
LIBBASE_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libbase
LIBFATFS_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libfatfs
LIBLITESPI_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitespi
LIBLITEDRAM_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitedram
LIBLITEETH_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/libliteeth
LIBLITESDCARD_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitesdcard
LIBLITESATA_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/liblitesata
BIOS_DIRECTORY=/home/adam/openfpga-litex2/litex/vendor/litex/litex/soc/software/bios
LTO=0
BIOS_CONSOLE_FULL=1