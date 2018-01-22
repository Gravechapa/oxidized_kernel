//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
pub mod area_frame_allocator;
mod paging;
pub mod heap_allocator;

use self::paging::PhysicalAddress;
pub use self::paging::remap_the_kernel;
use multiboot2::BootInformation;

pub fn init(mboot_info: &BootInformation)
{
    assert_has_not_been_called!("memory::init must be called only once");
    use memory::area_frame_allocator::AreaFrameAllocator;
    let mmap_tag = mboot_info.memory_map_tag()
        .expect("Memory map tag required");

    let elf_sections_tag = mboot_info.elf_sections_tag()
        .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr).min().unwrap();
    let kernel_end = elf_sections_tag.sections()
        .filter(|s| s.is_allocated()).map(|s| s.addr + s.size).max()
        .unwrap();

    println!("  kernel_start: {:#x}, kernel_end: {:#x}\n  mboot_start: {:#x}, mboot_end: {:#x}",
             kernel_start, kernel_end, mboot_info.start_address(), mboot_info.end_address());

    let mut frame_allocator = AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize,
        mboot_info.start_address(), mboot_info.end_address(),
        mmap_tag.memory_areas());

    let mut active_table = remap_the_kernel(&mut frame_allocator, mboot_info);

    use self::paging::Page;
    use {HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE - 1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page)
        {
            active_table.map(page, paging::entry::EntryFlags::WRITABLE, &mut frame_allocator);
        }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame 
{
    number: usize,
}

pub const PAGE_SIZE: usize = 4096;

pub trait FrameAllocator 
{
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

impl Frame 
{
    fn range_inclusive(start: Frame, end: Frame) -> FrameIter
    {
        FrameIter
            {
                start: start,
                end: end,
            }
    }

    fn containing_address(address: usize) -> Frame
    {
        Frame{number: address / PAGE_SIZE}
    }

    fn start_address(&self) -> PhysicalAddress
    {
        self.number * PAGE_SIZE
    }

    fn clone(&self) -> Frame
    {
        Frame {number: self.number}
    }
}

struct FrameIter
{
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter
{
    type Item = Frame;

    fn next(&mut self) -> Option<Frame>
    {
        if self.start <= self.end
            {
                let frame = self.start.clone();
                self.start.number += 1;
                Some(frame)
            }
        else
            {
                None
            }
    }
}