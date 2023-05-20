default rel

global _start
global multiboot_header_addr
extern kernel_bootstrap

section .multiboot2  align=32
start:
    dd 0xE85250D6 ; magic
    dd 0 ; architecture
    dd end - start ; header length
    dd 0 - (0xe85250d6 + end - start)
    ; end tag
    dw 0 ; type 0
    dw 0 
    dd 0 ; size 0
end:

;-----------------------
section .bss.init.stack nobits align=4096
stack_end:
    resb 4096
stack_start:

;-----------------------
section .bss.init.paging nobits align=4096
pml4e_table:
    resb 4096
pdpe_table_first_gb:
    resb 4096
pde_table_first_gb:
    resb 4096
pdpe_early_kernel_space:
    resb 4096
pde_early_kernel_space:
    resb 4096

;-----------------------
section .data.init.gdt_table align=4096
default rel
gdt_table:
    .null:
        dq 0
    .code: equ $ - gdt_table
        dw 0xffff
        dw 0
        db 0
        db 0b10011010
        db 0b11001111
        db 0
    .data: equ $ - gdt_table
        dw 0xffff
        dw 0
        db 0
        db 0b10010010
        db 0b11001111
        db 0
    .vga: equ $ - gdt_table
        dw 0x0fa0
        dw 0x8000
        db 0x0b
        db 0b10010010
        db 0b01000000
        db 0
    .code64: equ $ - gdt_table
        dd 0
        dd (1 << 21) | (1 << 15) | (1 << 12) | (1 << 11) | (1 << 10)
    .data64: equ $ - gdt_table
        dd 0
        dd (1 << 15) | (1 << 12)
    .register:
        dw $ - gdt_table - 1
        dd gdt_table
errors:
    .no_multiboot: db 'Multiboot2 not supported by bootloader', 0
    .halting: db 'Halting processor, goodbye !', 0
    .long_mode_failed: db 'Failed to enable long mode', 0

;-----------------------
section .data align=32
multiboot_header_addr:
    dd 01

;-----------------------
section .text32 align=32
default rel
bits 32

_start:
    cld
    cli
 
_kernel_start:
    ; check multiboot2
    mov esi, errors.no_multiboot
    cmp eax, 0x36d76289 ; multiboot2 magic number
    jne early_panic

    ; save multiboot header address
    mov [multiboot_header_addr], ebx
    
    ; setup protected mode GDT
    lgdt [rel gdt_table.register]
    jmp gdt_table.code:init_paging

init_paging:
    ; enabling PAE
    mov eax, cr4
    or eax, 1 << 5 | 1 << 7
    mov cr4, eax

    ; we will map the first GB of memory here, so we only need
    ; one entry in the pml4e and pdpe table
    mov eax, pdpe_table_first_gb
    or eax, 0b11
    mov [pml4e_table], eax

    ; and we will also map the last GB, which 

    mov eax, pde_table_first_gb
    or eax, 0b11
    mov [pdpe_table_first_gb], eax

    mov ecx, 0
.loop:
    mov eax, ecx
    shl eax, 21
    or eax, 0b11 | (1 << 7); PAGE_SIZE + READ_WRITE + PRESENT

    mov [pde_table_first_gb + 8 * ecx], eax 

    inc ecx
    cmp ecx, 512
    jne .loop
   
    ; load pml4e paging table
    mov eax, pml4e_table
    mov cr3, eax

    ; enable long mode
    mov ecx, 0xc0000080 ; EFER address
    rdmsr
    or eax, 1 << 8 ; Long mode enable
    or eax, 1 << 11 ; no execute enable
    wrmsr
   
    ; enable paging (and activate long mode)
    mov eax, cr0
    or eax, 1 << 31
    mov cr0, eax

    ; check long mode
    mov ecx, 0xc0000080
    rdmsr
    mov esi, errors.long_mode_failed
    test eax, 1 << 10
    jz early_panic 

    jmp gdt_table.code64:_start64

    mov esi, errors.halting
early_panic:
    mov edx, 0xb8000
    mov ah, 0b100
    .loop:
        mov al, [esi]
        cmp al, 0
        je .end_loop
        mov [edx], ax
        add edx, 2
        inc esi
        jmp .loop
    .end_loop:
halt_processor:
    hlt
    jmp halt_processor


section .text align=64
bits 64
; 64 bits entry points
_start64:
    ; setup stack
    xor rbp, rbp
    mov rsp, stack_start
    push 0
    push 0
	call kernel_bootstrap
    ret ; return to 0, effectively crashing

