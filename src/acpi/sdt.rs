use memory::{MemoryController, Frame};
use memory::EntryFlags;

#[derive(Copy, Clone, Debug)]
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
    pub fn init(address: usize, memory_controller: &mut MemoryController) -> Sdt
    {
        memory_controller.identity_map(Frame::containing_address(address),
                                       EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
        let sdt = unsafe {*(address as *const Sdt)};
        for frame in Frame::range_inclusive(Frame::containing_address(address + 4096),
                                            Frame::containing_address(address + sdt.length as usize))
            {
                memory_controller.identity_map(frame, EntryFlags::PRESENT | EntryFlags::NO_EXECUTE);
            }
        sdt
    }

    pub fn check(& self)
    {
        let mut checksum:i8 = 0;
        for i in 0..4
            {
                checksum += self.signature[i] as i8;
            }
        for i in 0..4
            {
                checksum += ((self.length >> (i * 8)) & 0xff) as i8;
            }
        checksum += self.revision as i8;
        checksum += self.checksum as i8;
        for i in 0..6
            {
                checksum += self.oem_id[i] as i8;
            }
        for i in 0..8
            {
                checksum += self.oem_table_id[i] as i8;
            }
        for i in 0..4
            {
                checksum += ((self.oem_revision >> (i * 8)) & 0xff) as i8;
            }
        for i in 0..4
            {
                checksum += ((self.creator_id >> (i * 8)) & 0xff) as i8;
            }
        for i in 0..4
            {
                checksum += ((self.creator_revision >> (i * 8)) & 0xff) as i8;
            }
        assert!(checksum == 0, "SDT check: FAIL");
    }
}