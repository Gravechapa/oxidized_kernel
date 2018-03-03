use super::sdt::Sdt;

bitflags!
{
    pub struct MAcpiFlags: u32
    {
        const PCAT_COMPAT = 1 << 0;
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct InterruptControllerHeader
{
    pub structure_type: u8,
    pub length: u8,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct  IOApic
{
    pub header: InterruptControllerHeader,
    pub io_apic_id: u8,
    reserved: u8,
    pub io_apic_address: u32,
    pub global_system_interrupt_base: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct  InterruptSourceOverride
{
    pub header: InterruptControllerHeader,
    pub bus: u8,
    pub source: u8,
    pub global_system_interrupt: u32,
    pub flags: u16,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
pub struct Madt
{
    header: Sdt,
    local_interrupt_controller_address: u32,
    flags: u32,
}


impl Madt
{
    pub fn new(sdt: &'static Sdt) -> &mut Madt
    {
        unsafe {&mut*(sdt as *const Sdt as *mut Madt)}
    }
}