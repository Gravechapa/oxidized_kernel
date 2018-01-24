use x86_64::registers::msr::{IA32_STAR, IA32_LSTAR, wrmsr, rdmsr};

const IA32_CSTAR: u32 = 0xC000_0083;
pub unsafe fn init()
{
    //let value: u64 = 0;
    //wrmsr(IA32_STAR, value | (0b11 << 48));
    wrmsr(IA32_LSTAR, syscall as u64);
    wrmsr(IA32_CSTAR, syscall as u64);
}

#[naked]
extern fn syscall()
{
    //unsafe{asm!("":"={rax}"(number), "={rdi}"(first):::"intel", "volatile");}
    println!("syscall");
    unsafe{asm!("sysret"
                :
                :
                :
                :"intel", "volatile");}
}