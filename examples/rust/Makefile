build:
	# set DEP_LV_CONFIG_PATH=`pwd`
	# cargo build --release --features build_dep
	cargo build --release
	cargo objcopy --release -- -O binary rust.bin
	cargo objdump --release -- -d > output.txt