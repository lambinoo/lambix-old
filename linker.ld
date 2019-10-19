ENTRY(_start)

SECTIONS {
    . = 1M;
    kernel_start_addr = .;
    .multiboot2 : {
        KEEP(*(.multiboot2))
    }

    .bootdata : {
        *(.bootdata, .bootdata.*)
    }

    .data : {
        *(data.rel.ro .data.rel.ro.*)
    }

    .bss : {
        *(.bss)
        *(.stack)
    }

    .text : {
        *(.text32 .text32.*)
        *(.text .text.*)
    }
    kernel_end_addr = .;
}

