.PHONY: fps minibreak minimal rtc vblank

fps:
	cd examples/rough_fps && \
	cargo build --release && \
	cargo objcopy --release -- -O binary ../../rust.bin

interact:
	cd examples/interact && \
	cargo build --release && \
	cargo objcopy --release -- -O binary ../../rust.bin

minibreak:
	cd examples/minibreak && \
	cargo build --release && \
	cargo objcopy --release -- -O binary ../../rust.bin

minimal:
	cd examples/minimal && \
	cargo build --release && \
	cargo objcopy --release -- -O binary ../../rust.bin

rtc:
	cd examples/rtc_id && \
	cargo build --release && \
	cargo objcopy --release -- -O binary ../../rust.bin

vblank:
	cd examples/vblank && \
	cargo build --release && \
	cargo objcopy --release -- -O binary ../../rust.bin
