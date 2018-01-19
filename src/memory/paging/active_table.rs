//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)

use super::entry::EntryFlags;
use super::inactive_table::InactivePageTable;
use super::temporary_page;
pub use super::mapper::Mapper;
use core::ops::{Deref, DerefMut};
use memory::Frame;

pub struct ActivePageTable
{
    mapper: Mapper,
}

impl Deref for ActivePageTable
{
    type Target = Mapper;

    fn deref(&self) -> &Mapper
    {
        &self.mapper
    }
}

impl DerefMut for ActivePageTable
{
    fn deref_mut(&mut self) -> &mut Mapper
    {
        &mut self.mapper
    }
}


impl ActivePageTable
{
    unsafe fn new() -> ActivePageTable
    {
        ActivePageTable
            {
                mapper: Mapper::new(),
            }
    }

    /// Временно заменяет указатель 512-ой точки входа таблицы p4 на указатель на неактивную таблицу
    /// и исполняет переданную функцию
    pub fn with<F>(&mut self, table: &mut InactivePageTable,
                   temporary_page: &mut temporary_page::TemporaryPage, func: F)
        where F: FnOnce(&mut Mapper)
    {

        use x86_64::instructions::tlb;
        use x86_64::registers::control_regs;
        {
            //cr3 backup
            let backup = Frame::containing_address(
                control_regs::cr3().0 as usize);

            // map temporary_page to current p4 table
            let p4_table = temporary_page.map_table_frame(backup.clone(), self);

            // overwrite recursive mapping
            self.p4_mut()[511].set(table.get_frame(),
                                   EntryFlags::PRESENT | EntryFlags::WRITABLE);
            tlb::flush_all();

            // execute func in the new context
            func(self);

            // restore recursive mapping to original p4 table
            p4_table[511].set(backup, EntryFlags::PRESENT | EntryFlags::WRITABLE);
            tlb::flush_all();
        }
        temporary_page.unmap(self);
    }
}
