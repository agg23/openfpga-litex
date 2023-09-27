.section .init
 
.option norvc
 
.type _start, @function
.global _start
_start:
	.cfi_startproc

	li  x1, 0
  li  x2, 0
  li  x3, 0
  li  x4, 0
  li  x5, 0
  li  x6, 0
  li  x7, 0
  li  x8, 0
  li  x9, 0
  li  x10,0
  li  x11,0
  li  x12,0
  li  x13,0
  li  x14,0
  li  x15,0
  li  x16,0
  li  x17,0
  li  x18,0
  li  x19,0
  li  x20,0
  li  x21,0
  li  x22,0
  li  x23,0
  li  x24,0
  li  x25,0
  li  x26,0
  li  x27,0
  li  x28,0
  li  x29,0
  li  x30,0
  li  x31,0
 
.option push
.option norelax
	la gp, _global_pointer
.option pop
 
	/* Reset satp */
	csrw satp, zero
 
	/* Setup stack */
	la sp, _stack_top
 
	/* Clear the BSS section */
	la t5, _bss_start
	la t6, _bss_end
bss_clear:
	sw zero, 0 (t5)
	addi t5, t5, 8
	bltu t5, t6, bss_clear
 
	la t0, main
	csrw mepc, t0
 
	/* Jump to kernel! */
	tail main
 
	.cfi_endproc
 
.end
