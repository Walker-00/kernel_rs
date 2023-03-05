global long_mode_start ; global long_mode_start
extern kernel_main ; call kernel_main

section .text ; text section
bits 64 ; 64 bits assembly
long_mode_start: ; long mode start lable
    ; load null into all data segment registers
    mov ax, 02
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

	call kernel_main ; call the kernel_main func
    hlt ; halt the cpu