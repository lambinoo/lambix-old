ENTRY(_start)

__eh_frame_hdr_start = ADDR(.eh_frame_hdr);
__eh_frame_hdr_end = ADDR(.eh_frame_hdr) + SIZEOF(.eh_frame_hdr);
__eh_frame_start = ADDR(.eh_frame);
__eh_frame_end = ADDR(.eh_frame) + SIZEOF(.eh_frame);


SECTIONS {
    . = 2M;
    kernel_start_addr = .;

    .eh_frame_hdr : {
        *(.eh_frame_hdr, .eh_frame_hdr.*)
    }

    .eh_frame : {
        *(.eh_frame, .eh_frame.*)
    }

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

