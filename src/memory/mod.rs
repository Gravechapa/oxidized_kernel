//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
pub mod area_frame_allocator;
mod paging;

use self::paging::PhysicalAddress;
pub use self::paging::remap_the_kernel;

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