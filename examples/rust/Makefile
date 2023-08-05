build:
	cargo build --release
	cargo objcopy --release -- -O binary rust.bin
	cargo objdump --release -- -d > output.txt