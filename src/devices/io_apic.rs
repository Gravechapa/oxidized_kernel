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
    if (madt.get_flags() & 1) == 1
        {
            disable_8259pic();
        }

    let mut io_apics: Vec<IOApicController> = Vec::new();
    for interrupt_controller in madt.get_iter()
        {
            println!("{:?}", interrupt_controller);
            match interrupt_controller
                {
                    InterruptController::IOApic(io_apic) =>
                        io_apics.push(IOApicController::new(io_apic, memory_controller)),
                    _ => (),
                };
        }

    io_apics[0].set_redir_entry(EntryData
        {
            vector: 0x21,
            delivery_mode: 0,
            dest_mode: 0,
            delivery_status: 0,
            pin_polarity: 0,
            remote_irr: 0,
            trigger_mode: 0,
            mask: 0,
            destination: 0,
        }, 1);

    /*for i in 0..16
        {
            println!("{:?}", io_apics[0].get_redir_entry(i));
        }*/
    //println!("{:?}", io_apics);

    use x86_64::instructions::interrupts::enable;
    unsafe {enable();}
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

    pub fn get_redir_count(&self) -> u8
    {
        self.redir_entry_count
    }

    pub fn get_redir_entry(&self, number: u8) -> EntryData
    {
        if number >= self.redir_entry_count
            {
                panic!("devices::io_apic::IOApicController::get_redir_entry number: out of range");
            }
        unsafe
            {
                let lower_dword = self.read(IOAPICREDTBL + 2 * number);
                let upper_dword = self.read(IOAPICREDTBL + 2 * number + 1);

                let entry = RedirectionEntry
                    {
                        entry_parts: RedirectionEntryParts
                            {
                                lower_dword,
                                upper_dword,
                            },
                    };
                EntryData::new(entry.entry)
            }
    }

    pub fn set_redir_entry(&self, entry: EntryData, number: u8)
    {
        let entry = RedirectionEntry
            {
                entry: entry.get_data(),
            };
        unsafe
            {
                self.write(IOAPICREDTBL + 2 * number + 1, entry.entry_parts.upper_dword);
                self.write(IOAPICREDTBL + 2 * number, entry.entry_parts.lower_dword);
            }
    }

}

#[derive(Debug)]
pub struct EntryData
{
    vector: u8,
    delivery_mode: u8,
    dest_mode: u8,
    delivery_status: u8,
    pin_polarity: u8,
    remote_irr: u8,
    trigger_mode: u8,
    mask: u8,
    destination: u8,
}

impl EntryData
{
    pub fn new(entry: u64) -> EntryData
    {
        EntryData
            {
                vector: entry as u8,
                delivery_mode: (entry >> 8 & 0b111) as u8,
                dest_mode: (entry >> 11 & 0b1) as u8,
                delivery_status: (entry >> 12 & 0b1) as u8,
                pin_polarity: (entry >> 13 & 0b1) as u8,
                remote_irr: (entry >> 14 & 0b1) as u8,
                trigger_mode: (entry >> 15 & 0b1) as u8,
                mask: (entry >> 16 & 0b1) as u8,
                destination: (entry >> 56) as u8,
            }
    }

    pub fn get_data(&self) -> u64
    {
        (self.vector as u64) | (self.delivery_mode as u64) << 8 | (self.dest_mode as u64) << 11 |
            (self.delivery_status as u64) << 12 | (self.pin_polarity as u64) << 13 | (self.remote_irr as u64) << 14 |
            (self.trigger_mode as u64) << 15 | (self.mask as u64) << 16 | (self.destination as u64) << 56
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RedirectionEntryParts
{
    lower_dword: u32,
    upper_dword: u32,
}

pub union RedirectionEntry
{
    entry: u64,
    entry_parts: RedirectionEntryParts,
}

fn disable_8259pic()
{
    use x86_64::instructions::port::outb;
    unsafe
        {
            // starts the initialization sequence (in cascade mode)
            outb(0x20, 0x11);
            outb(0xa0, 0x11);

            // Master PIC vector offset
            // Slave PIC vector offset
            outb(0x21, 0x20);
            outb(0xa1, 0x28);

            // Tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
            // Tell Slave PIC its cascade identity (0000 0010)
            outb(0x21, 4);
            outb(0xa1, 2);

            // 8086/88 (MCS-80/85) mode
            outb(0x21, 1);
            outb(0xa1, 1);

            //Disable the PIC
            outb(0x21, 0xff);
            outb(0xa1, 0xff);
        }
}