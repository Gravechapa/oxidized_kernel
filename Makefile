arch ?= x86_64
kernel := build/oxidized_kernel-$(arch).bin
iso := build/os-$(arch).iso
target ?= $(arch)-oxidized_kernel
rust_os := target/$(target)/debug/liboxidized_kernel.a

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
	
debug: $(iso)
	@nohup qemu-system-x86_64 -cdrom $(iso) -s -S > /dev/null 2>&1 &
	
#lldb: debug
#	@rust-lldb "build/oxidized_kernel-x86_64.bin" -o "gdb-remote 1234"
	
gdb: debug
	@rust-gdb "build/oxidized_kernel-x86_64.bin" -ex "target remote :1234"
	
cgdb: debug
	@cgdb -d rust-gdb "build/oxidized_kernel-x86_64.bin" -ex "target remote :1234"


iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p build/isofiles/boot/grub
	@cp $(kernel) build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) build/isofiles/boot/grub
	@grub-mkrescue -d /usr/lib/grub/i386-pc -o $(iso) build/isofiles 2> /dev/null
	@rm -r build/isofiles


$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n --gc-sections -T $(linker_script) -o $(kernel) \
	$(assembly_object_files) $(rust_os)

kernel:
	@~/.cargo/bin/xargo build --target $(target)
	
# compile assembly files
build/arch/$(arch)/%.o: assembler_src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@ 
