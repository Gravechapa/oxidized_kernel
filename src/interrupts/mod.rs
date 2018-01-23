//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
mod exceptions;
mod gdt;

use x86_64::structures::idt::Idt;
use self::exceptions::*;
use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtualAddress;
use spin::Once;


static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

const DOUBLE_FAULT_IST_INDEX: usize = 0;

lazy_static!
{
    static ref IDT: Idt =
    {
        let mut idt = Idt::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe
            {
                idt.double_fault.set_handler_fn(double_fault_handler)
                        .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
            }

        idt
    };
}
pub fn init(memory_controller: &mut MemoryController)
{
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(||
        {
            let mut tss = TaskStateSegment::new();
            tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(
                double_fault_stack.top());
            tss
        });

    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(||
        {
            let mut gdt = gdt::Gdt::new();
            code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
            tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
            gdt
        });
    gdt.load();

    unsafe
        {
            // reload code segment register
            set_cs(code_selector);
            // load TSS
            load_tss(tss_selector);
        }


    IDT.load();
}