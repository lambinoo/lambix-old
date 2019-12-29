ENTRY(_start)

SECTIONS {
    . = 1M;
    kernel_start_addr = .;

    .multiboot2 : {
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
        *(.bss, .bss.*)
        *(.init, .init.*)
    }

    .text : {
        *(.text32 .text32.*)
        *(.text .text.*)
    }

    kernel_end_addr = .;
}

