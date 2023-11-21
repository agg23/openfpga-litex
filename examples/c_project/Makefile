build:
	riscv64-unknown-elf-gcc -Wall -Wextra -march=rv32i -mabi=ilp32 -c -mcmodel=medany kernel.c -o kernel.o -ffreestanding
	riscv64-unknown-elf-as -march=rv32i -mabi=ilp32 -c entry.S -o entry.o
	riscv64-unknown-elf-gcc -march=rv32i -mabi=ilp32 -T linker.ld -nostdlib kernel.o entry.o -o kernel.elf
	riscv64-unknown-elf-objcopy -O binary kernel.elf kernel.bin