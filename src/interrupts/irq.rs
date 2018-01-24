use x86_64::structures::idt::ExceptionStackFrame;
pub extern "x86-interrupt" fn keyboard_irq(stack_frame: &mut ExceptionStackFrame)
{
    println!("keyboard");
}