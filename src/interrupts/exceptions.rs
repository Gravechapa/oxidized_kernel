//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
use x86_64::structures::idt::ExceptionStackFrame;

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}