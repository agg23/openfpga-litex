export OUT_DIR="../build/litex/rust" && export BUILD_DIR="../build"

main:
	cargo build --bin rust --release
	cargo objcopy --bin rust --release -- -O binary rust.bin
	# cargo objdump --release -- -d > output.txt

audio:
	cargo build --bin audio --release
	cargo objcopy --bin audio --release -- -O binary rust.bin

rtc:
	cargo build --bin rtc --release
	cargo objcopy --bin rtc --release -- -O binary rust.bin