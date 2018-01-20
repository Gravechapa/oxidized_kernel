//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
use memory::Frame;
use super::temporary_page::TemporaryPage;
use super::active_table::ActivePageTable;
use super::entry::EntryFlags;

pub struct InactivePageTable
{
    pub p4_frame: Frame,
}

impl InactivePageTable
{
    pub fn new(frame: Frame, active_table: &mut ActivePageTable,
               temporary_page: &mut TemporaryPage) -> InactivePageTable
    {
        {
            let table = temporary_page.map_table_frame(frame.clone(),
                                                       active_table);
            // now we are able to zero the table
            table.clear();
            // set up recursive mapping for the table
            table[511].set(frame.clone(), EntryFlags::PRESENT | EntryFlags::WRITABLE);
        }
        temporary_page.unmap(active_table);

        InactivePageTable {p4_frame: frame}
    }
}