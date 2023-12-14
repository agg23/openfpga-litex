INCLUDE output_format.ld
ENTRY(_start)

__DYNAMIC = 0;

INCLUDE regions.ld

SECTIONS
{
	.text :
	{
		_ftext = .;
		/* Make sure init_asm files come first, and they, and the isr */
		/* don't get disposed of by greedy optimisation */
		*init_asm*(.text)
		KEEP(*init_asm*(.text))
		KEEP(*(.text.isr))

		*(.text .stub .text.* .gnu.linkonce.t.*)
		_etext = .;
	} > main_ram

	.rodata :
	{
		. = ALIGN(8);
		_frodata = .;
		*(.rodata .rodata.* .gnu.linkonce.r.*)
		*(.rodata1)
		*(.got .got.*)
		*(.toc .toc.*)
		. = ALIGN(8);
		_erodata = .;
	} > main_ram

	.data :
	{
		. = ALIGN(8);
		_fdata = .;
		*(.data .data.* .gnu.linkonce.d.*)
		*(.data1)
		_gp = ALIGN(16);
		*(.sdata .sdata.* .gnu.linkonce.s.*)
		. = ALIGN(8);
		_edata = .;
	} > main_ram

	.bss :
	{
		. = ALIGN(8);
		_fbss = .;
		*(.dynsbss)
		*(.sbss .sbss.* .gnu.linkonce.sb.*)
		*(.scommon)
		*(.dynbss)
		*(.bss .bss.* .gnu.linkonce.b.*)
		*(COMMON)
		. = ALIGN(8);
		_ebss = .;
		_end = .;
	} > main_ram
}

PROVIDE(_fstack = ORIGIN(main_ram) + LENGTH(main_ram));

PROVIDE(_fdata_rom = LOADADDR(.data));
PROVIDE(_edata_rom = LOADADDR(.data) + SIZEOF(.data));
PROVIDE(_emain_ram = ORIGIN(main_ram) + LENGTH(main_ram)); 

/* Start heap at end of .bss segment */
PROVIDE (__heap_start = _end);
PROVIDE (__heap_end = _fstack - (DEFINED(__stack_size) ? __stack_size : 0x800));
PROVIDE (__heap_size = __heap_end - __heap_start);
