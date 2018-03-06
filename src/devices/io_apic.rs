use acpi::{AcpiController, Madt};
use memory::MemoryController;

pub fn init(acpi_controller: &AcpiController, memory_controller: &mut MemoryController)
{
    let madt = Madt::new(acpi_controller.get_entries_map().get("APIC").expect("MADT not found"));
    for interrupt_controller in madt.get_iter()
        {
            println!("{:?}", interrupt_controller);
        }
}