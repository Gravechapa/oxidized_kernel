//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
pub mod entry;
mod table;
mod active_table;
mod temporary_page;
mod inactive_table;
mod mapper;

use memory::{PAGE_SIZE, Frame, FrameAllocator};
use multiboot2::BootInformation;
use self::active_table::ActivePageTable;
use self::inactive_table::InactivePageTable;
use self::temporary_page::TemporaryPage;


/// Количество точек входа на странице
const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct PageIter
{
    start: Page,
    end: Page,
}

impl Iterator for PageIter
{
    type Item = Page;

    fn next(&mut self) -> Option<Page>
    {
        if self.start <= self.end
            {
                let page = self.start;
                self.start.number += 1;
                Some(page)
            }
        else
            {
                None
            }
    }
}

/// Виртуальная страница
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page
{
    number: usize,
}

impl Page
{
    /// Возвращает страницу виртуальной памяти в случае если адрес валиден
    pub fn containing_address(address: VirtualAddress) -> Page
    {
        assert!(address < 0x0000_8000_0000_0000 ||
                    address >= 0xffff_8000_0000_0000,
                "invalid address: 0x{:x}", address);
        Page {number: address / PAGE_SIZE}
    }

    /// Возвращает виртуальный адрес начала страницы
    fn start_address(&self) -> usize
    {
        self.number * PAGE_SIZE
    }

    /// Возвращает индекс точки фхода на странице p4
    fn p4_index(&self) -> usize {
        (self.number >> 27) & 0o777
    }
    /// Возвращает индекс точки фхода на странице p3
    fn p3_index(&self) -> usize {
        (self.number >> 18) & 0o777
    }
    /// Возвращает индекс точки фхода на странице p2
    fn p2_index(&self) -> usize {
        (self.number >> 9) & 0o777
    }
    /// Возвращает индекс точки фхода на странице p1
    fn p1_index(&self) -> usize {
        (self.number >> 0) & 0o777
    }

    pub fn range_inclusive(start: Page, end: Page) -> PageIter
    {
        PageIter
            {
                start: start,
                end: end,
            }
    }

}

pub fn remap_the_kernel<A>(allocator: &mut A, boot_info: &BootInformation) -> ActivePageTable
    where A: FrameAllocator
{
    let mut temporary_page = TemporaryPage::new(Page {number: 0xbabebeef}, allocator);

    let mut active_table = unsafe {ActivePageTable::new()};
    let mut new_table =
        {
            let frame = allocator.allocate_frame().expect("no more frames");
            InactivePageTable::new(frame, &mut active_table, &mut temporary_page)
        };

    active_table.with(&mut new_table, &mut temporary_page, |mapper|
        {
            use self::entry::EntryFlags;

            let elf_sections_tag = boot_info.elf_sections_tag()
                .expect("Memory map tag required");

            for section in elf_sections_tag.sections()
                {


                    if !section.is_allocated()
                        {
                            // section is not loaded to memory
                            continue;
                        }

                    assert!(section.start_address() % PAGE_SIZE == 0,
                            "sections need to be page aligned");

                    println!("  mapping section at addr: {:#x}, size: {:#x}",
                             section.addr, section.size);

                    let flags = EntryFlags::from_elf_section_flags(section);

                    let start_frame = Frame::containing_address(section.start_address());
                    let end_frame = Frame::containing_address(section.end_address() - 1);
                    for frame in Frame::range_inclusive(start_frame, end_frame)
                        {
                            mapper.identity_map(frame, flags, allocator);
                        }
                }

            // identity map the VGA text buffer
            let vga_buffer_frame = Frame::containing_address(0xb8000);
            mapper.identity_map(vga_buffer_frame, EntryFlags::WRITABLE, allocator);

            // identity map the multiboot info structure
            let mboot_start = Frame::containing_address(boot_info.start_address());
            let mboot_end = Frame::containing_address(boot_info.end_address() - 1);
            for frame in Frame::range_inclusive(mboot_start, mboot_end)
                {
                    mapper.identity_map(frame, EntryFlags::PRESENT, allocator);
                }
        });

    let old_table = active_table.switch(new_table);
    println!("  Kernel remapped");

    // turn the old p4 page into a guard page
    let old_p4_page = Page::containing_address(old_table.p4_frame.start_address());
    active_table.unmap(old_p4_page, allocator);
    println!("  guard page at {:#x}", old_p4_page.start_address());

    active_table
}