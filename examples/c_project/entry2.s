.section .init;
.global _start;
_start:
  la gp, _global_pointer;
  la sp, _stack_top;
  j main;
