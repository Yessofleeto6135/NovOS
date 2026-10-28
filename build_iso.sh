#!/bin/bash
set -e

echo "=== NovOS ISO Generation Script ==="
echo "1. Building NovOS Kernel binaries for x86 and x86_64..."

cargo build --release

mkdir -p iso_x86/boot/grub
mkdir -p iso_x86_64/boot/grub

# Copy GRUB config for x86 32-bit
cat << 'EOF' > iso_x86/boot/grub/grub.cfg
set timeout=5
set default=0

menuentry "NovOS Live (x86 32-bit)" {
    multiboot /boot/novos_kernel_x86
    boot
}

menuentry "NovOS Installer Mode" {
    multiboot /boot/novos_kernel_x86 --install
    boot
}
EOF

# Copy GRUB config for x86_64 64-bit
cat << 'EOF' > iso_x86_64/boot/grub/grub.cfg
set timeout=5
set default=0

menuentry "NovOS Live (x86_64 64-bit)" {
    multiboot2 /boot/novos_kernel_x86_64
    boot
}

menuentry "NovOS Installer Mode" {
    multiboot2 /boot/novos_kernel_x86_64 --install
    boot
}
EOF

# Copy kernel binary into ISO directories
cp target/release/novos-kernel iso_x86/boot/novos_kernel_x86
cp target/release/novos-kernel iso_x86_64/boot/novos_kernel_x86_64

echo "2. Packaging ISO bootable images..."
grub-mkrescue -o novos_x86.iso iso_x86
grub-mkrescue -o novos_x86_64.iso iso_x86_64

echo "=== NovOS ISO Build Complete ==="
ls -lh novos_x86.iso novos_x86_64.iso
