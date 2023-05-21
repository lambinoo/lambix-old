ENTRY(_start)

SECTIONS {
    . = 2M;
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

    . = ALIGN(8);
    PROVIDE(__eh_frame_hdr = .);
    .eh_frame_hdr : {
	    KEEP (*(.eh_frame_hdr)) *(.eh_frame_hdr.*)
    }

    . = ALIGN(8);
    PROVIDE(__eh_frame = .);
    .eh_frame : {
	    KEEP (*(.eh_frame)) *(.eh_frame.*)
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

