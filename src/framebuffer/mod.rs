mod vga_text_buffer;

use core::fmt::Write;
use core::fmt;

const  BUFFER_HEIGHT: usize = 25;
const  BUFFER_WIDTH: usize = 80;

macro_rules! print {
    ($($arg:tt)*) => ({
        $crate::framebuffer::print(format_args!($($arg)*));
    });
}

pub fn print(args: fmt::Arguments)
{
    WRITER.lock().write_fmt(args).unwrap();
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

use core::ptr::Unique;
use spin::Mutex;
pub static WRITER: Mutex<vga_text_buffer::Writer> = Mutex::new(vga_text_buffer::Writer::new());

pub fn clear_screen()
{
    for _ in 0..BUFFER_HEIGHT
        {
            println!("");
        }
}