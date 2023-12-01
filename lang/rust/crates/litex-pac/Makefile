build:
	rm -Rf src/svd
	mkdir -p src/svd
	svd2rust -i ../../../../litex/pocket.svd --output-dir ./src/svd/ --target riscv --make_mod
	bindgen ../../../../litex/build/litex/software/include/generated/soc.h -o src/constants.rs