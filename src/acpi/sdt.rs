use memory::{MemoryController, Frame};
use memory::EntryFlags;

pub const SDT_SIZE: u8 = 36;

#[derive(Debug)]
#[repr(packed)]
pub struct Sdt
{
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32
}

impl Sdt
{
    pub fn init(address: usize, memory_controller: &mut MemoryController) -> &'static Sdt
    {
        if !memory_controller.check(address)
            {
                memory_controller.identity_map(Frame::containing_address(address),
                                               EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
            }
        let sdt = unsafe {&*(address as *const Sdt)};
        for frame in Frame::range_inclusive(Frame::containing_address(address + 4096),
                                            Frame::containing_address(address + sdt.length as usize))
            {
                if !memory_controller.check(address)
                    {
                        memory_controller.identity_map(frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
                    }
            }
        sdt.check();
        sdt
    }

    pub fn check(& self)
    {
        assert!(self.length != 0);
        let mut checksum:i16 = 0;
        let pointer = self as *const Sdt as *const i8;
        for i in 0..self.length
            {
                unsafe {checksum += *pointer.offset(i as isize) as i16;}

            }
        assert!(checksum & 0xff == 0, "SDT check: FAIL");
    }
}