use x86_64::structures::idt::ExceptionStackFrame;
use x86_64::instructions::port::inb;
use devices::apic::eoi;

pub extern "x86-interrupt" fn keyboard_irq(stack_frame: &mut ExceptionStackFrame)
{
    unsafe
        {
            print!("KB_scancode:{} ", get_scancode());
            eoi();
        }
}

unsafe fn get_scancode() -> u8
{
    let mut c=0u8;
    loop
        {
            if inb(0x60) != c
                {
                    c = inb(0x60);
                    if c > 0
                        {
                            return c;
                        }
                }
        }
}

/*fn get_char() -> u8
{
    return scancode[getScancode()+1];
}*/