mod writer;
mod font_8x16;
use multiboot2::BootInformation;

use core::fmt::Write;
use core::fmt;

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::framebuffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments)
{
    let guard = GUARD.lock();
    unsafe
        {
            match WRITER
                {
                    Some(ref mut writer) => writer.write_fmt(args).unwrap(),
                    None => (),
                }
        }
    drop(guard);
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

use spin::Mutex;
static GUARD: Mutex<()> = Mutex::new(());
pub static mut WRITER: Option<writer::Writer> = None;

pub fn clear_screen()
{
    let guard = GUARD.lock();
    unsafe
        {
            match WRITER
                {
                    Some(ref mut writer) => writer.clear_screen(),
                    None => (),
                }
        }
    drop(guard);
}

pub fn init(mboot_info: &BootInformation)
{
    if mboot_info.framebuffer_tag().is_some()
        {
            let framebuffer = mboot_info.framebuffer_tag().expect("Framebuffer!");
            let guard = GUARD.lock();
            unsafe{WRITER = writer::Writer::new(framebuffer);}
            drop(guard);
        }
    clear_screen();
}