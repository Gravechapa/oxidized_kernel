#![feature(lang_items)]
#![no_std]
#![feature(unique)]
#![feature(const_fn)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(global_allocator)]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(core_intrinsics)]
#![feature(i128_type)]

#![feature(asm)]

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
extern crate lazy_static;
extern crate bit_field;
#[macro_use]
extern crate raw_cpuid;



#[macro_use]
mod framebuffer;
mod memory;
mod interrupts;
mod syscall;
mod devices;

use memory::heap_allocator::BumpAllocator;
use devices::apic;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START,
                                                          HEAP_START + HEAP_SIZE);


#[no_mangle]
pub extern fn rust_main(mboot_address: usize, test: usize)
{
    enable_extended_feature();
    enable_write_protect_bit();
    let mboot_info = unsafe{multiboot2::load(mboot_address)};

    framebuffer::clear_screen();

    let mut memory_controller = memory::init(mboot_info);

    framebuffer::init(mboot_info);

    interrupts::init(&mut memory_controller);

    unsafe {syscall::init()};

    apic::init();

    use alloc::String;
    let rsdp = mboot_info.acpi_2_tag().expect("").get_rsdp();
    unsafe{println!("{:?}\n {}\n {}", rsdp, String::from_raw_parts( rsdp.signature.as_ptr() as *mut u8, 8, 8),
                    String::from_raw_parts(rsdp.oem_id.as_ptr() as *mut u8, 6, 6));}


    /*let framebuffer = mboot_info.framebuffer_tag().expect("");
    let frame_color = framebuffer.get_direct_rgb_color().expect("");
    let mut address: *mut u32 = framebuffer.framebuffer_addr as *mut _;
    for i in 0..framebuffer.framebuffer_width * framebuffer.framebuffer_height
    {
        unsafe {*address = 0xffffff}
        address = (address as u64 + 4) as *mut u32;
    }*/
    //framebuffer::rgb_framebuffer::draw_char(framebuffer.framebuffer_addr, framebuffer.framebuffer_pitch,
    //                                        49, 0xffffff, 0);

    /*let mut a:i64 = 10;
    unsafe{asm!("
                 syscall"
                :
                :
                :
                :"intel", "volatile");}
    println!("{}",a);*/

  /*  fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();*/

    use alloc::boxed::Box;
    let heap_test = Box::new(42);

    println!("{}", test);
    let test1 = 0o177777_777_777_777_777_0002 as *mut i64;
    /*unsafe {
        *test1 = 10;
        println!("{}", *test1);
    }*/

    loop{println!("dddddddddddddddddddddddddddddddddddddddd")}
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

fn enable_extended_feature()
{
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    let syscall_bit = 1;
    unsafe
        {
            let efer = rdmsr(IA32_EFER);
            wrmsr(IA32_EFER, efer | nxe_bit | syscall_bit);
        }
}

fn enable_write_protect_bit()
{
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe {cr0_write(cr0() | Cr0::WRITE_PROTECT)};
}