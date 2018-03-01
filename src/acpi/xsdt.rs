use memory::MemoryController;
use super::sdt::Sdt;

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
}