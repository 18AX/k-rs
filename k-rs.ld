OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start)

STACK_SIZE = 0x400000;
HEAP_SIZE = 0x400000;

SECTIONS
{
    . = 0xffffffff80200000;
    
    .text : {
        *(.text*)
    }
 
    .rodata : {
        *(.rodata*)
    }
 
    .data : {
        *(.data*)
    }
 
    .bss : {
        *(COMMON)
        *(.bss*)
    }

    .heap : {
        . = ALIGN(4096);
        PROVIDE(heap_begin = .);
        . = . + HEAP_SIZE;
        PROVIDE(heap_end = .);
    }

    .stack : {
        . = ALIGN(4096);
        PROVIDE(stack_begin = .);
        . = . + STACK_SIZE;
        PROVIDE(stack_end = .);
    }
      
    PROVIDE(_end = .);
}