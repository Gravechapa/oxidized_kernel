//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
use x86_64::structures::idt::ExceptionStackFrame;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn double_fault_handler(stack_frame: &mut ExceptionStackFrame,
                                               _error_code: u64)
{
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop {}
}
