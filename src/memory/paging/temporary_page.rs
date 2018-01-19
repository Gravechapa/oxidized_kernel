//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
use super::Page;
use super::VirtualAddress;
use super::active_table::ActivePageTable;
use memory::Frame;
use memory::FrameAllocator;
use super::table::{Table, Level1};

struct TinyAllocator([Option<Frame>; 3]);

impl TinyAllocator
{
    fn new<A>(allocator: &mut A) -> TinyAllocator
        where A: FrameAllocator
    {
        let mut f = || allocator.allocate_frame();
        let frames = [f(), f(), f()];
        TinyAllocator(frames)
    }
}

impl FrameAllocator for TinyAllocator
{
    fn allocate_frame(&mut self) -> Option<Frame>
    {
        for frame_option in &mut self.0
            {
                if frame_option.is_some()
                    {
                        return frame_option.take();
                    }
            }
        None
    }

    fn deallocate_frame(&mut self, frame: Frame)
    {
        for frame_option in &mut self.0
            {
                if frame_option.is_none()
                    {
                        *frame_option = Some(frame);
                        return;
                    }
            }
        panic!("Tiny allocator can hold only 3 frames.");
    }
}

/// Временная страница для новой таблицы
pub struct TemporaryPage
{
    page: Page,
    allocator: TinyAllocator,
}

impl TemporaryPage
{
    pub fn new<A>(page: Page, allocator: &mut A) -> TemporaryPage
        where A: FrameAllocator
    {
        TemporaryPage
            {
                page: page,
                allocator: TinyAllocator::new(allocator),
            }
    }

    /// Связывает временную таблицу с физической областью в активной таблице
    /// Возвращает ссылку на только что привязанную таблицу
    /// Maps the temporary page to the given page table frame in the active
    /// table. Returns a reference to the now mapped table.
    pub fn map_table_frame(&mut self, frame: Frame,
                           active_table: &mut ActivePageTable) -> &mut Table<Level1>
    {
        unsafe {&mut *(self.map(frame, active_table) as *mut Table<Level1>)}
    }

    /// Связывает временную таблицу с физической областью в активной таблице
    /// Возвращает адрес начала временной страницы
    /// Maps the temporary page to the given frame in the active table.
    /// Returns the start address of the temporary page.
    pub fn map(&mut self, frame: Frame, active_table: &mut ActivePageTable) -> VirtualAddress
    {
        use super::entry::EntryFlags;

        assert!(active_table.translate_page(self.page).is_none(),
                "temporary page is already mapped");
        active_table.map_to(self.page, frame, EntryFlags::WRITABLE, &mut self.allocator);
        self.page.start_address()
    }

    /// Отвязывает временную страницу от активной таблицы
    /// Unmaps the temporary page in the active table.
    pub fn unmap(&mut self, active_table: &mut ActivePageTable)
    {
        active_table.unmap(self.page, &mut self.allocator)
    }
}
