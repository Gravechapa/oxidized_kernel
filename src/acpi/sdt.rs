use memory::{MemoryController, Frame};
use memory::EntryFlags;

#[derive(Debug)]
#[repr(packed)]
pub struct Sdt
{
    signature: [u8; 4],
    length: u32,
    revision: u8,
    checksum: u8,
    oem_id: [u8; 6],
    oem_table_id: [u8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32
}

impl Sdt
{
    pub fn init(address: usize, memory_controller: &mut MemoryController) -> &'static Sdt
    {
        memory_controller.identity_map(Frame::containing_address(address),
                                       EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
        let sdt = unsafe {&*(address as *const Sdt)};
        sdt.check();
        for frame in Frame::range_inclusive(Frame::containing_address(address + 4096),
                                            Frame::containing_address(address + sdt.length as usize))
            {
                memory_controller.identity_map(frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
            }
        sdt
    }

    pub fn check(& self)
    {
        let mut checksum:i16 = 0;
        let pointer = self as *const Sdt as *const i8;
        for i in 0..self.length
            {
                unsafe {checksum = *pointer.offset(i as isize) as i16;}
            }
        assert!(checksum & 0xff == 0, "SDT check: FAIL");
    }
}