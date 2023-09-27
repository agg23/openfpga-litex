ENTRY(_start);
 
. = 0x00000000;
 
SECTIONS {
	/* Include entry point at start of binary */
	.text : ALIGN(4K) {
		*(.init);
		*(.text);
	}
	.bss : ALIGN(4K) {
		PROVIDE(_bss_start = .);
		*(.bss);
		. += 4096;
		PROVIDE(_stack_top = .);
		. += 4096;
		PROVIDE(_global_pointer = .);
		PROVIDE(_bss_end = .);
	}
	.rodata : ALIGN(4K) {
		*(.rodata);
	}
	.data : ALIGN(4K) {
		*(.data);
	}
}