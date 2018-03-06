use super::sdt::{Sdt, SDT_SIZE};

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
struct IOApic
{
    pub header: InterruptControllerHeader,
    pub io_apic_id: u8,
    reserved: u8,
    pub io_apic_address: u32,
    pub global_system_interrupt_base: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(packed)]
struct InterruptSourceOverride
{
    pub header: InterruptControllerHeader,
    pub bus: u8,
    pub source: u8,
    pub global_system_interrupt: u32,
    pub flags: u16,
}

#[derive(Debug)]
pub enum InterruptController
{
    IOApic(&'static IOApic),
    InterruptSourceOverride(&'static InterruptSourceOverride),
    Another(u8),
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

    pub fn get_iter(&self) -> InterruptControllerIter
    {
        InterruptControllerIter
            {
                madt: self,
                offset: 8 + SDT_SIZE as usize,
            }
    }
}

#[derive(Debug)]
pub struct InterruptControllerIter<'a>
{
    madt: &'a Madt,
    offset: usize,
}

impl<'a> Iterator for InterruptControllerIter<'a>
{
    type Item = InterruptController;
    fn next(&mut self) -> Option<Self::Item>
    {
        if self.offset != self.madt.header.length as usize
            {
                let ic_header =
                    unsafe{&*(((self.madt as *const Madt as usize) + self.offset) as *const InterruptControllerHeader)};

                let interrupt_controller = match ic_header.structure_type
                    {
                        1 => InterruptController::IOApic(
                            unsafe{&*((ic_header as *const InterruptControllerHeader) as *const IOApic)}),
                        2 => InterruptController::InterruptSourceOverride(
                            unsafe{&*((ic_header as *const InterruptControllerHeader) as *const InterruptSourceOverride)}),
                        _ => InterruptController::Another(ic_header.structure_type),
                    };
                self.offset += ic_header.length as usize;

                Some(interrupt_controller)
            }
        else
            {
                None
            }

    }
}