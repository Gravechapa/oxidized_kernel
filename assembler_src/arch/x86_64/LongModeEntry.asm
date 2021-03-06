;Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
global long_mode_start

section .text
bits 64
long_mode_start:

    ; clear all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    
    extern rust_main
    call rust_main

    ; print `OKAY` to screen
    mov rax, 0x2f592f412f4b2f4f
    mov qword [0xb8000], rax
    hlt 
