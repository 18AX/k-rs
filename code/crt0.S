.global _start
_start:
    .extern stack_end
    .extern heap_begin
    .extern heap_end
    .extern k_main

    movq $stack_end,%rsp
    movq $stack_end,%rbp

    call k_main