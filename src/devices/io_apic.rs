use acpi::AcpiController;
use acpi::madt::{Madt, InterruptController, IOApic};
use memory::{MemoryController, Frame};
use memory::EntryFlags;
use alloc::Vec;
use core::ptr;

pub fn init(acpi_controller: &AcpiController, memory_controller: &mut MemoryController)
{
    assert_has_not_been_called!("devices::io_apic::init must be called only once");
    let madt = Madt::new(acpi_controller.get_entries_map().get("APIC").expect("MADT not found"));
    let mut io_apics: Vec<IOApicController> = Vec::new();
    for interrupt_controller in madt.get_iter()
        {
            match interrupt_controller
                {
                    InterruptController::IOApic(io_apic) =>
                        io_apics.push(IOApicController::new(io_apic, memory_controller)),
                    _ => (),
                };
        }
    println!("{:?}", io_apics);
}

//regs
static IOAPICID: u8 = 0x00;
static IOAPICVER: u8 = 0x01;
static IOAPICARB: u8 = 0x02;
static IOAPICREDTBL: u8 = 0x10;


#[derive(Copy, Clone, Debug)]
struct IOApicController
{
    io_apic: &'static IOApic,
    apic_id: u8,
    apic_ver: u8,
    redir_entry_count: u8,
}

impl IOApicController
{
    pub fn new(io_apic: &'static IOApic, memory_controller: &mut MemoryController) -> IOApicController
    {
        let mut io_apic_controller = IOApicController
            {
                io_apic,
                apic_id: 0,
                apic_ver: 0,
                redir_entry_count: 0,
            };
        io_apic_controller.init(memory_controller);
        io_apic_controller
    }

    fn init(&mut self, memory_controller: &mut MemoryController)
    {
        if !memory_controller.check(self.io_apic.io_apic_address as usize)
            {
                memory_controller.identity_map(
                    Frame::containing_address(self.io_apic.io_apic_address as usize),
                    EntryFlags::WRITABLE | EntryFlags::NO_EXECUTE);
            }
        unsafe
            {
                self.apic_id = ((self.read(IOAPICID) >> 24) & 0xf0) as u8;
                self.apic_ver = self.read(IOAPICVER) as u8;
                self.redir_entry_count = ((self.read(IOAPICVER) >> 16) + 1) as u8;
            }
    }

    unsafe fn read(&self, reg: u8) -> u32
    {
        ptr::write_volatile(self.io_apic.io_apic_address as *mut u32, reg as u32);
        ptr::read_volatile((self.io_apic.io_apic_address + 0x10) as *const u32)
    }

    unsafe fn write(&self, reg: u8, data: u32)
    {
        ptr::write_volatile(self.io_apic.io_apic_address as *mut u32, reg as u32);
        ptr::write_volatile((self.io_apic.io_apic_address + 0x10) as *mut u32, data);
    }
}