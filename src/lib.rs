#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate bitflags;
extern crate x86_64;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate once;



#[macro_use]
mod vga_text_buffer;
mod memory;

use memory::heap_allocator::BumpAllocator;
pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START,
                                                          HEAP_START + HEAP_SIZE);


#[no_mangle]
pub extern fn rust_main(mboot_address: usize, test: usize)
{
    enable_nxe_bit();
    enable_write_protect_bit();
    let mboot_info = unsafe{multiboot2::load(mboot_address)};

    vga_text_buffer::clear_screen();

    memory::init(mboot_info);

    use alloc::boxed::Box;
    let heap_test = Box::new(42);

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

fn enable_nxe_bit()
{
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe
        {
            let efer = rdmsr(IA32_EFER);
            wrmsr(IA32_EFER, efer | nxe_bit);
        }
}

fn enable_write_protect_bit()
{
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe {cr0_write(cr0() | Cr0::WRITE_PROTECT)};
}