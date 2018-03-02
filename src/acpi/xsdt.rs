use memory::MemoryController;
use super::sdt::{Sdt, SDT_SIZE};

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

    pub fn get_entries(&self, memory_controller: &mut MemoryController)
    {
        let entries = ((self.sdt as *const Sdt as usize) + SDT_SIZE as usize) as *mut u64;
        for i in 0..(self.sdt.length - SDT_SIZE as u32) / 8
            {
                let address = unsafe{entries.offset(i as isize) as usize};
                let sdt = Sdt::init(address, memory_controller);
            }
    }
}