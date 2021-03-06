arch ?= x86_64
kernel := build/oxidized_kernel-$(arch).bin
kernel_debug := build/oxidized_kernel_debug-$(arch).bin
iso := build/os-$(arch).iso
efi_iso := build/os-$(arch)-efi.iso
iso_debug := build/os_debug-$(arch).iso
efi_iso_debug := build/os_debug-$(arch)-efi.iso
target ?= $(arch)-oxidized_kernel
rust_os := target/$(target)/release/liboxidized_kernel.a
rust_os_debug := target/$(target)/debug/liboxidized_kernel.a

linker_script := assembler_src/arch/$(arch)/linker.ld
grub_cfg := assembler_src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard assembler_src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst assembler_src/arch/$(arch)/%.asm, \
	build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: $(kernel)

clean:
	@rm -r build

run: $(iso)
	@qemu-system-x86_64 -d int -no-reboot -cdrom $(iso)
	
run_efi: $(efi_iso)
	@qemu-system-x86_64 -d int -no-reboot -cdrom $(efi_iso) -drive if=pflash,format=raw,readonly,file=./ovmf_code_x64.bin -drive if=pflash,format=raw,file=./ovmf_vars_x64.bin
	
debug: $(iso_debug)
	@nohup qemu-system-x86_64 -cdrom $(iso_debug) -s -S > /dev/null 2>&1 &
	
debug_efi: $(efi_iso_debug)
	@nohup qemu-system-x86_64 -cdrom $(efi_iso_debug) -s -S -drive if=pflash,format=raw,readonly,file=./ovmf_code_x64.bin -drive if=pflash,format=raw,file=./ovmf_vars_x64.bin > /dev/null 2>&1 &
	
	
#lldb: debug
#	@rust-lldb "build/oxidized_kernel-x86_64.bin" -o "gdb-remote 1234"
	
gdb: debug
	@rust-gdb "build/oxidized_kernel-x86_64.bin" -ex "target remote :1234"
	
cgdb: debug
	@cgdb -d rust-gdb "build/oxidized_kernel-x86_64.bin" -ex "target remote :1234"


gdb_efi: debug_efi
	@rust-gdb "build/oxidized_kernel-x86_64.bin" -ex "target remote :1234"
	
cgdb_efi: debug_efi
	@cgdb -d rust-gdb "build/oxidized_kernel-x86_64.bin" -ex "target remote :1234"

iso: $(iso)

efi_iso: $(efi_iso)

iso_debug: $(iso_debug)

efi_iso_debug: $(efi_iso_debug)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@pkgdatadir=/usr/share/grub grub-mkrescue -d /usr/lib/grub/i386-pc -o $(iso) build/isofiles
	@rm -r build/isofiles
	
$(efi_iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@pkgdatadir=/usr/share/grub grub-mkrescue -d /usr/lib/grub/x86_64-efi -o $(efi_iso) build/isofiles
	@rm -r build/isofiles

$(iso_debug): $(kernel_debug) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel_debug) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@pkgdatadir=/usr/share/grub grub-mkrescue -d /usr/lib/grub/i386-pc -o $(iso_debug) build/isofiles
	@rm -r build/isofiles
	
$(efi_iso_debug): $(kernel_debug) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel_debug) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@pkgdatadir=/usr/share/grub grub-mkrescue -d /usr/lib/grub/x86_64-efi -o $(efi_iso_debug) build/isofiles
	@rm -r build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
	$(assembly_object_files) $(rust_os)
	
$(kernel_debug): kernel_debug $(rust_os_debug) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel_debug) \
	$(assembly_object_files) $(rust_os_debug)

kernel:
	@~/.cargo/bin/xargo build --release --target $(target)
	
kernel_debug:
	@~/.cargo/bin/xargo build --target $(target)
	
# compile assembly files
build/arch/$(arch)/%.o: assembler_src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@ 
