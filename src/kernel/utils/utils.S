global div_by_zero_pls

section .text align=64
bits 64

div_by_zero_pls:
	mov rdx, 5
	mov rax, 5
	mov rbx, 0
	div rbx
	ret

