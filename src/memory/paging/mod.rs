//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
mod entry;
mod table;
mod active_table;
mod temporary_page;
mod inactive_table;
mod mapper;

use memory::PAGE_SIZE;


/// Количество точек входа на странице
const ENTRY_COUNT: usize = 512;

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

/// Виртуальная страница
#[derive(Debug, Clone, Copy)]
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

}