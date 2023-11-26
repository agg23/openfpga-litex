PACKAGES=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata bios
PACKAGE_DIRS=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libc $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libcompiler_rt $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libbase $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libfatfs $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitespi $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitedram $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libliteeth $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitesdcard $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitesata $(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/bios
LIBS=libc libcompiler_rt libbase libfatfs liblitespi liblitedram libliteeth liblitesdcard liblitesata
TRIPLE=riscv64-unknown-elf
CPU=vexriscv
CPUFAMILY=riscv
CPUFLAGS= -march=rv32i2p0_mafdc -mabi=ilp32d -D__vexriscv__ -DUART_POLLING
CPUENDIANNESS=little
CLANG=0
CPU_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/cores/cpu/vexriscv_smp
SOC_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc
PICOLIBC_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/pythondata-software-picolibc/pythondata_software_picolibc/data
PICOLIBC_FORMAT=integer
COMPILER_RT_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/pythondata-software-compiler_rt/pythondata_software_compiler_rt/data
export BUILDINC_DIRECTORY
BUILDINC_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/build/litex/software/include
LIBC_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libc
LIBCOMPILER_RT_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libcompiler_rt
LIBBASE_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libbase
LIBFATFS_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libfatfs
LIBLITESPI_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitespi
LIBLITEDRAM_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitedram
LIBLITEETH_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/libliteeth
LIBLITESDCARD_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitesdcard
LIBLITESATA_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/liblitesata
BIOS_DIRECTORY=$(LITEX_ROOT_DIRECTORY)/vendor/litex/litex/soc/software/bios
LTO=0
BIOS_CONSOLE_FULL=1