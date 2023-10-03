build:
	export OUT_DIR="../build/analogue_pocket/rust" && export BUILD_DIR="../build" && cargo build --release
	cargo objcopy --release -- -O binary rust.bin
	cargo objdump --release -- -d > output.txt