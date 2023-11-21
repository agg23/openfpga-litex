/*
Source: https://sourceware.org/binutils/docs/ld/Scripts.html
Source: https://docs.rs/riscv-rt/latest/riscv_rt/
Source: https://docs.rust-embedded.org/embedonomicon/memory-layout.html
*/

/* Define memory layout (RAM only architecture) */
MEMORY {
    /*RAM : ORIGIN = 0x00000000, LENGTH =  256K*/   /* 1K = 1 KiBi = 1024 bytes */
    /*RAM : ORIGIN = 0x00000000, LENGTH =  64K*/
    RAM : ORIGIN = 0x00000000, LENGTH =  75K
}

/* Define sections */
SECTIONS {
    . = ORIGIN(RAM);
    .init : { *(.init) } > REGION_INIT /* Set init as first section, this is the program's entry point if the PC starts at zero */
    .text : { *(.text .text.*) } > REGION_TEXT
    .rodata : { *(.rodata) } > REGION_RODATA
    .data : { *(.data) } > REGION_DATA
    . = ALIGN(4);
}

/* Set aliases (since this is a RAM only architecture, all regions are in RAM) */
REGION_ALIAS("REGION_INIT", RAM);
REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);