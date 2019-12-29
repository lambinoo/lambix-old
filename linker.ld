ENTRY(_start)

SECTIONS {
    . = 1M;
    . = ALIGN(4096);
    kernel_start_addr = .;

    .multiboot2 :  {
        KEEP(*(.multiboot2))
    }

    .data : {
        *(.data, .data.*)
        *(.data.rel.ro .data.rel.ro.*)
    }

    .rodata : {
        *(.rodata, .rodata.*)
    }

    .bss : {
        *(COMMON)
        *(.bss, .bss.*)
    }

    .text : {
        *(.text .text.*)
    }

    .text32 : {
        *(.text32 .text32.*)
    }

    . = ALIGN(4096);
    kernel_end_addr = .;
}

