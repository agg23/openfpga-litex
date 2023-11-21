/* Copyright (c) 2020 Sonal Pinto       */
/* SPDX-License-Identifier: Apache-2.0  */

OUTPUT_ARCH( "riscv" )

MEMORY {
    bootrom  (rx) : ORIGIN = 0x00000000, LENGTH = 1K
    ram      (rwx): ORIGIN = 0x00010000, LENGTH = 128K
    system   (rw) : ORIGIN = 0x00800000, LENGTH = 8M
}

ENTRY(_start)

SECTIONS
{
    .text (ORIGIN(bootrom)) :
    {   
        *(.init)
        *(.text)
        *(.rodata)
        PROVIDE(_etext = .);
    } > bootrom

    .data (ORIGIN(ram)): ALIGN(4)
    {
        . = ALIGN(4);

        PROVIDE(_global_pointer = . + 0x800);

        PROVIDE(_sdata = .);
        *(.data)
        . = ALIGN(4);
        PROVIDE(_edata = .);
        /* AT specifies the load address of this section, using the next free address from the "ATed" section */
    } > ram AT> bootrom

    .bss (NOLOAD) : ALIGN(4)
    {   
         . = ALIGN(4);
        PROVIDE(_bss_start = .);
        *(.bss)
        *(COMMON)
         . = ALIGN(4);
        PROVIDE(_bss_end = .);
    } > ram

    /* Stack Pointer - End of Memory */
    PROVIDE(_stack_top = ORIGIN(ram) + LENGTH(ram));
}