//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
use super::entry::*;
use super::ENTRY_COUNT;
use core::ops::{Index, IndexMut};
use core::marker::PhantomData;
use memory::FrameAllocator;

pub trait TableLevel {}
pub trait HierarchicalLevel: TableLevel
{
    type NextLevel: TableLevel;
}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

impl HierarchicalLevel for Level4
{
    type NextLevel = Level3;
}
impl HierarchicalLevel for Level3
{
    type NextLevel = Level2;
}
impl HierarchicalLevel for Level2
{
    type NextLevel = Level1;
}

/// virtual address
/// 16-sign extension // 9-p4 // 9-p3 // 9-p2 // 9-p1 // 12-offset
pub const P4: *mut Table<Level4> = 0o177777_777_777_777_777_0000 as *mut _;

/// Таблица виртуальной памяти
pub struct Table<L: TableLevel>
{
    /// Массив точек входа
    entries: [Entry; ENTRY_COUNT],
    /// Уровень таблицы
    level: PhantomData<L>,
}

impl<L> Table<L> where L: TableLevel
{
    /// Очистка таблицы
    pub fn clear(&mut self)
    {
        for entry in self.entries.iter_mut()
            {
                entry.set_unused();
            }
    }
}

impl<L> Table<L> where L: HierarchicalLevel
{

    /// Создает новые страницы виртуальной памяти
    pub fn next_table_create<A>(&mut self, index: usize, allocator: &mut A) -> &mut Table<L::NextLevel>
        where A: FrameAllocator
    {
        if self.next_table(index).is_none()
            {
                assert!(!self.entries[index].flags().contains(EntryFlags::HUGE_PAGE),
                        "mapping code does not support huge pages");
                let frame = allocator.allocate_frame().expect("no frames available");
                self.entries[index].set(frame, EntryFlags::PRESENT | EntryFlags::WRITABLE);
                self.next_table_mut(index).unwrap().clear();
            }
        self.next_table_mut(index).unwrap()
    }

    /// Возвращает адрес таблицы следующего уровня
    fn next_table_address(&self, index: usize) -> Option<usize>
    {
        let entry_flags = self[index].flags();
        if entry_flags.contains(EntryFlags::PRESENT) && !entry_flags.contains(EntryFlags::HUGE_PAGE)
            {
                let table_address = self as *const _ as usize;
                Some((table_address << 9) | (index << 12))
            }
        else
            {
                None
            }
    }

    /// Возвращает таблицу следующего уровня
    pub fn next_table(&self, index: usize) -> Option<&Table<L::NextLevel>>
    {
        self.next_table_address(index)
            .map(|address| unsafe { &*(address as *const _) })
    }

    /// Возвращает таблицу следующего уровня с возможностью изменения
    pub fn next_table_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>>
    {
        self.next_table_address(index)
            .map(|address| unsafe { &mut *(address as *mut _) })
    }
}

impl<L> Index<usize> for Table<L> where L: TableLevel
{
    type Output = Entry;

    /// Итератор по таблице
    fn index(&self, index: usize) -> &Entry
    {
        &self.entries[index]
    }
}

impl<L> IndexMut<usize> for Table<L> where L: TableLevel
{
    /// Итератор по таблице с возможностью изменения
    fn index_mut(&mut self, index: usize) -> &mut Entry
    {
        &mut self.entries[index]
    }
}