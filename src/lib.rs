#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;


#[macro_use]
mod vga_text_buffer;
mod memory;


#[no_mangle]
pub extern fn rust_main(mboot_address: usize, test: usize)
{
    let mboot_info = unsafe{multiboot2::load(mboot_address)};
    let mmap_tag = mboot_info.memory_map_tag()
    .expect("Memory map tag required");

    use vga_text_buffer::*;
    clear_screen();
    println!("  memory areas:");
    for area in mmap_tag.memory_areas()
    {
        println!("    start: 0x{:x}, length: 0x{:x}",
        area.base_addr, area.length);
    }

    let elf_sections_tag = mboot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    println!("  kernel sections:");
    for section in elf_sections_tag.sections() 
    {
        println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
            section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
    .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
    .max().unwrap();

    let mboot_start = mboot_address;
    let mboot_end = mboot_start + (mboot_info.total_size as usize);

    println!("    kernel_start: {}, kernel_end: {}\n    mboot_start: {}, mboot_end: {}",
    kernel_start, kernel_end, mboot_start, mboot_end);

    use memory::area_frame_allocator::*;
    use memory::FrameAllocator;

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, mboot_start,
        mboot_end, mmap_tag.memory_areas());

    println!("{}", test);
    let test1 = 0o177777_777_777_777_777_0002 as *mut i64;
    /*unsafe {
        *test1 = 10;
        println!("{}", *test1);
    }*/

    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
    loop{}
}
