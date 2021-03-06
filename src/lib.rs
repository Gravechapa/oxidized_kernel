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
mod acpi;

use memory::heap_allocator::BumpAllocator;
use devices::apic;

pub const HEAP_START: usize = 0o_000_001_000_000_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

#[global_allocator]
static HEAP_ALLOCATOR: BumpAllocator = BumpAllocator::new(HEAP_START,
                                                          HEAP_START + HEAP_SIZE);


#[no_mangle]
pub extern "C" fn rust_main(mboot_address: usize)
{
    enable_extended_feature();
    enable_write_protect_bit();
    let mboot_info = unsafe{multiboot2::load(mboot_address)};

    let mut memory_controller = memory::init(mboot_info);

    interrupts::init(&mut memory_controller);

    framebuffer::init(mboot_info);

    let acpi_controller = acpi::init(mboot_info, &mut memory_controller);

    devices::init(&acpi_controller, &mut memory_controller);

    unsafe {syscall::init()};

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
    /*
    use alloc::boxed::Box;
    let heap_test = Box::new(42);
    */
    loop{ unsafe{asm!("
                 hlt"
                :
                :
                :
                :"intel", "volatile");}
    }
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