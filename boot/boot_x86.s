; NovOS Bootloader Assembly Stage (Multiboot 1 & Multiboot 2 header for x86 and x86_64)
[bits 32]

section .multiboot
align 4
    ; Multiboot 1 Header
    MAGIC       equ 0x1BADB002
    FLAGS       equ 0x00000007 ; Align modules, mem info, video mode
    CHECKSUM    equ -(MAGIC + FLAGS)

    dd MAGIC
    dd FLAGS
    dd CHECKSUM
    dd 0, 0, 0, 0, 0
    dd 0 ; 0 = Linear graphics mode
    dd 1024 ; Width
    dd 768  ; Height
    dd 32   ; Depth

section .text
global _start
extern novos_kernel_main_x86
extern novos_kernel_main_x86_64

_start:
    cli
    mov esp, stack_top

    ; Pass multiboot magic and info pointer to Rust kernel entry
    push ebx ; Multiboot info struct pointer
    push eax ; Multiboot magic

    call novos_kernel_main_x86

.hang:
    hlt
    jmp .hang

section .bss
align 16
stack_bottom:
    resb 65536 ; 64 KB Stack
stack_top:
