pub mod apic;
mod io_apic;

use acpi::AcpiController;
use memory::MemoryController;

pub fn init(acpi_controller: &AcpiController, memory_controller: &mut MemoryController)
{
    apic::init();
    io_apic::init(acpi_controller, memory_controller);
}