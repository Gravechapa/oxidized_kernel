//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
mod exceptions;
mod gdt;
mod irq;

use x86_64::structures::idt::Idt;
use self::exceptions::*;
use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::{VirtualAddress, PrivilegeLevel};
use spin::Once;
use self::irq::*;

static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new();

const DOUBLE_FAULT_IST_INDEX: usize = 0;

lazy_static!
{
    static ref IDT: Idt =
    {
        let mut idt = Idt::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.divide_by_zero.set_handler_fn(divide_by_zero_handler);
        idt.debug.set_handler_fn(debug_handler);
        idt.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded.set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available.set_handler_fn(device_not_available_handler);
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present.set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point.set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point.set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);
        idt.security_exception.set_handler_fn(security_exception_handler);
        unsafe
            {
                idt.double_fault.set_handler_fn(double_fault_handler)
                        .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
            }

        idt.interrupts[1].set_handler_fn(keyboard_irq);

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