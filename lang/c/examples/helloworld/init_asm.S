.global main
.global _start

_start:
  // Make sure stack starts where we want
  la sp, _fstack

  // Initialize bss block to 0
  la a0, _fbss
  la a1, _ebss
bss_loop:
  beq a0,a1,bss_done
  sw zero,0(a0)
  add a0,a0,4
  j bss_loop
bss_done:
  call main
