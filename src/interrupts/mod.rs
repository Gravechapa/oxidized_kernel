//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)
mod exceptions;

use x86_64::structures::idt::Idt;
use self::exceptions::*;

lazy_static!
{
    static ref IDT: Idt =
    {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}
pub fn init()
{
    IDT.load();
}