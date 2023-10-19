export OUT_DIR="../build/litex/rust" && export BUILD_DIR="../build"

main:
	cargo build --release
	cargo objcopy --release -- -O binary rust.bin
	# cargo objdump --release -- -d > output.txt

audio:
	cargo build --bin audio --release
	cargo objcopy --bin audio --release -- -O binary rust.bin