use multiboot2::BootInformation;
use memory::MemoryController;
use alloc::BTreeMap;

use alloc::String;

mod gas;
mod sdt;
mod xsdt;
mod fadt;
pub mod madt;

use self::xsdt::Xsdt;
use self::fadt::Fadt;
use self::sdt::Sdt;

pub struct AcpiController
{
    xsdt: Xsdt,
    entries_map: BTreeMap<String, &'static Sdt>,
}

impl AcpiController
{
    pub fn get_entries_map(&self) -> &BTreeMap<String, &'static Sdt>
    {
        &self.entries_map
    }
}

pub fn init(mboot_info: &BootInformation, memory_controller: &mut MemoryController)
    -> AcpiController
{
    assert_has_not_been_called!("acpi::init must be called only once");
    println!("ACPI initing");
    let rsdp = mboot_info.acpi_2_tag().expect("RSDP not found").get_rsdp();
    let mut checksum:i16 = 0;

    /*unsafe{println!("{:?}\n {}\n {}", rsdp, String::from_raw_parts( rsdp.signature.as_ptr() as *mut u8, 8, 8),
                    String::from_raw_parts(rsdp.oem_id.as_ptr() as *mut u8, 6, 6));}*/

    //acpi v1 checksum
    for i in 0..8
        {
            checksum += rsdp.signature[i] as i8 as i16;
        }
    checksum += rsdp.checksum as i8 as i16;
    for i in 0..6
        {
            checksum += rsdp.oem_id[i] as i8 as i16;
        }
    checksum += rsdp.revision as i8 as i16;
    for i in 0..4
        {
            checksum += ((rsdp.rsdt_address >> (i * 8)) & 0xff) as i8 as i16;
        }
    assert!(checksum & 0xff == 0, "ACPIv1 RSDP check: FAIL");
    println!("ACPIv1 RSDP check: OK");

    //acpi v2 checksum
    for i in 0..4
        {
            checksum += ((rsdp.length >> (i * 8)) & 0xff) as i8 as i16;
        }
    for i in 0..8
        {
            checksum += ((rsdp.xsdt_address >> (i * 8)) & 0xff) as i8 as i16;
        }
    checksum += rsdp.extended_checksum as i8 as i16;
    for i in 0..3
        {
            checksum += rsdp.reserved[i] as i8 as i16;
        }
    assert!(checksum & 0xff == 0, "ACPIv2 RSDP check: FAIL");
    println!("ACPIv2 RSDP check: OK");

    unsafe{println!("ACPI OEM: {}", String::from_raw_parts(rsdp.oem_id.as_ptr() as *mut u8, 6, 6));}

    let xsdt = Xsdt::init(rsdp.xsdt_address as usize, memory_controller);
    let entries_map = xsdt.get_entries(memory_controller);

    /*
    let fadt = Fadt::new(entries_map.get("FACP").expect("FADT not found"));
    println!("{:?}\n", entries_map);
    //println!("{:?}", fadt);*/

    AcpiController
        {
            xsdt,
            entries_map,
        }
}