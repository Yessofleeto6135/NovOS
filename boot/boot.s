[bits]
section .boot

; Bootloader code here

org 0x7C00

start:
    ; Your bootloader entry point code
    cli
    mov ax, 0x0000
    mov ds, ax
    mov si, msg
    call print

hang:
    jmp hang

msg db 'The Starting of NovOS', 0

print:
    ; Code to print the message
    ret

; Boot signature
times 510-($-$$) db 0
 dw 0xAA55
