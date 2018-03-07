use core::intrinsics::{volatile_load, volatile_store};
use raw_cpuid::CpuId;
use x86_64::registers::msr::{IA32_APIC_BASE, wrmsr, rdmsr};
use x86_64::registers::msr;
use alloc::boxed::Box;

pub fn check_apic() -> bool
{
    CpuId::new().get_feature_info().unwrap().has_apic()
}

pub fn check_x2apic() -> bool
{
    CpuId::new().get_feature_info().unwrap().has_x2apic()
}

pub fn get_base_address() -> u64
{
    rdmsr(IA32_APIC_BASE) & 0xffffffffff000
}

pub unsafe fn eoi()
{
    if check_x2apic()
        {
            wrmsr(msr::IA32_X2APIC_EOI, 0);
        }
    else
        {
            *((get_base_address() + 0xb0) as *mut u32) = 0;
        }
}

pub fn init() /*-> Box<ApicController>*/
{
    assert_has_not_been_called!("devices::apic::init must be called only once");
    assert!(check_apic());

    let apic: Box<ApicController>;
    if check_x2apic()
        {
            unsafe{wrmsr(IA32_APIC_BASE, rdmsr(IA32_APIC_BASE) | 1 << 10);}
            apic = Box::new(X2apic::new());
        }
    else
        {

            apic = Box::new(Apic::new());
        }
    println!("apic version:{}", apic.get_version());
    apic.enable();
}

pub trait ApicController
{
    fn get_version(&self) -> u32;
    fn enable(&self);
}

struct Apic (u64);
impl Apic
{
    fn new() -> Apic
    {
        Apic(get_base_address())
    }

}

impl ApicController for Apic
{
    fn get_version(&self) -> u32
    {
        unsafe {volatile_load((self.0 + 0x30) as *const u32)}
    }

    fn enable(&self)
    {
        unsafe {volatile_store((self.0 + 0xf0) as *mut u32, 0x100)}
    }
}

struct X2apic();
impl X2apic
{
    fn new() -> X2apic
    {
        X2apic()
    }
}

impl ApicController for X2apic
{
    fn get_version(&self) -> u32
    {
        rdmsr(msr::IA32_X2APIC_VERSION) as u32
    }

    fn enable(&self)
    {
        unsafe{wrmsr(msr::IA32_X2APIC_SIVR, 0x100);}
    }
}