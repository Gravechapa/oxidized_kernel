use memory::MemoryController;
use super::sdt::{Sdt, SDT_SIZE};
use alloc::BTreeMap;
use alloc::String;

pub struct Xsdt
{
    sdt: &'static Sdt,
}

impl Xsdt
{
    pub fn init(address: usize, memory_controller: &mut MemoryController) -> Xsdt
    {
        let sdt = Sdt::init(address, memory_controller);
        println!("XSDT check: OK");
        Xsdt{sdt}
    }

    pub fn get_entries(&self, memory_controller: &mut MemoryController) -> BTreeMap<String, &'static Sdt>
    {
        let mut map = BTreeMap::new();
        let entries = ((self.sdt as *const Sdt as usize) + SDT_SIZE as usize) as *mut u64;
        for i in 0..(self.sdt.length - SDT_SIZE as u32) / 8
            {
                let address = unsafe{*entries.offset(i as isize) as usize};
                let sdt = Sdt::init(address, memory_controller);
                unsafe
                    {
                        map.insert(String::from_raw_parts(sdt.signature.as_ptr() as *mut u8,
                                                          4, 4), sdt);
                    }
            }
        map
    }
}