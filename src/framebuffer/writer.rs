//! Some code was borrowed from [Phil Opp's Blog](http://os.phil-opp.com/)

use super::font_8x16::*;
use multiboot2::FramebufferTag;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
#[repr(u8)]
pub enum Color
{
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}


#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode
{
    pub const fn new(foreground: Color, background: Color) -> ColorCode
    {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RgbColor
{
    foreground: u32,
    background: u32,
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar
{
    ascii_character: u8,
    color_code: ColorCode,
}

use core::ptr::Unique;

pub struct Writer
{
    column_position: u32,
    row_position: u32,
    addr: u64,
    pitch: u32,
    width: u32,
    height: u32,
    char_width: u8,
    char_height: u8,
    bpp: u8,
    fb_type: u8,
    color: CurrentColor,

}

union CurrentColor
{
    color_code: ColorCode,
    rgb: RgbColor,
}

impl Writer
{
    pub fn new(framebuffer: &FramebufferTag) -> Option<Writer>
    {
        match framebuffer.framebuffer_type
            {
                0 => None/* Indexed color unsupported */,
                1 => Some(Writer
                {
                    column_position: 0,
                    row_position: 0,
                    addr: framebuffer.framebuffer_addr,
                    pitch: framebuffer.framebuffer_pitch,
                    width: framebuffer.framebuffer_width,
                    height: framebuffer.framebuffer_height,
                    char_width: 8,
                    char_height: 16,
                    bpp: framebuffer.framebuffer_bpp,
                    fb_type: framebuffer.framebuffer_type,
                    color: CurrentColor
                        {rgb: RgbColor
                            {
                                foreground: 0x18F018,
                                background: 0,
                            }
                        },
                }),
                2 => {
                    ///////// Disable cursor ///////////
                    use x86_64::instructions::port::outb;
                    unsafe
                        {
                            outb(0x3D4, 0x0A);
                            outb(0x3D5, 0x20);
                        }
                    ////////////////////////////////////
                    Some(Writer
                    {
                        column_position: 0,
                        row_position: 0,
                        addr: framebuffer.framebuffer_addr,
                        pitch: framebuffer.framebuffer_pitch,
                        width: framebuffer.framebuffer_width,
                        height: framebuffer.framebuffer_height,
                        char_width: 1,
                        char_height: 1,
                        bpp: framebuffer.framebuffer_bpp,
                        fb_type: framebuffer.framebuffer_type,
                        color: CurrentColor
                            {
                                color_code: ColorCode::new(Color::Green, Color::Black),
                            },
                    })},
                _ => None,
            }
    }

    pub fn write_byte(&mut self, byte: u8)
    {
        match byte
            {
                b'\n' => self.new_line(),
                byte => {
                    if self.column_position >= (self.width / self.char_width as u32)
                        {
                            self.new_line();
                        }

                    self.draw(self.row_position, self.column_position, byte);
                    self.column_position += 1;
                }
            }
    }

    fn new_line(&mut self)
    {
        self.row_position += 1;
        self.column_position = 0;
        if self.row_position >= (self.height / self.char_height as u32)
            {
                self.shift();
            }
    }


    fn shift(&mut self)
    {
        use rlibc::memmove;
        let row_size = self.pitch * self.char_height as u32;
        let grid_height = self.height / self.char_height as u32;
        unsafe {memmove(self.addr as *mut u8,
                (self.addr + row_size as u64) as *mut u8,
                        (row_size * (grid_height - 1)) as usize);}
        self.row_position -= 1;
        self.clear_row(self.row_position);
    }

    fn clear_row(&self, row: u32)
    {
        for col in 0..self.width / self.char_width as u32
            {
                self.draw(row, col, b' ');
            }
    }

    pub fn clear_screen(&mut self)
    {
        for row in 0..self.height / self.char_height as u32
            {
                self.write_byte(b'\n');
            }
        self.row_position = 0;
        self.column_position = 0;
    }

    fn draw(&self, row: u32, col: u32, character: u8)
    {
        match self.fb_type
            {
                1 => match self.bpp
                    {
                        32 => self.draw_char_rgb_32(self.addr +
                                                       ((self.pitch * row * self.char_height as u32) +
                                                           (col * self.char_width as u32) * 4) as u64,
                                                   character,
                                                   unsafe {self.color.rgb.foreground},
                                                   unsafe {self.color.rgb.background}),
                        16 => self.draw_char_rgb_16(self.addr +
                                                        ((self.pitch * row * self.char_height as u32) +
                                                            (col * self.char_width as u32) * 2) as u64,
                                                    character,
                                                    convert_color_to_16(unsafe {self.color.rgb.foreground}),
                                                    convert_color_to_16(unsafe {self.color.rgb.background})),
                        8 => self.draw_char_rgb_8(self.addr +
                                                        ((self.pitch * row * self.char_height as u32) +
                                                            (col * self.char_width as u32)) as u64,
                                                    character,
                                                  convert_color_to_8(unsafe {self.color.rgb.foreground}),
                                                  convert_color_to_8(unsafe {self.color.rgb.background})),
                        _ => (),
                    },
                2 => self.draw_char_ega(row, col, character),
                _=> (),
            }
    }

    fn draw_char_rgb_32(&self, address: u64, character: u8, foreground_colour: u32, background_colour: u32)
    {
        let mut address = address;
        let char_start = character as usize * 16;
        let font_data_for_char = &FONT_8X16[char_start..char_start + 16];
        let packed_foreground: u128 = ((foreground_colour as u128) << 96) |
            ((foreground_colour as u128) << 64) |
            ((foreground_colour as u128) << 32) |
            foreground_colour as u128;
        let packed_background: u128 = ((background_colour as u128) << 96) |
            ((background_colour as u128) << 64) |
            ((background_colour as u128) << 32) |
            background_colour as u128;

        for row in 0..16
            {
                let row_data = font_data_for_char[row];
                let mask1 = LOOKUP_TABLE_32BIT[(row_data >> 4) as usize];
                let mask2 = LOOKUP_TABLE_32BIT[(row_data & 0x0F) as usize];
                unsafe {*(address as *mut u128) = (packed_foreground & mask1) | (packed_background & !mask1);}
                unsafe {*((address + 16) as *mut u128) = (packed_foreground & mask2) | (packed_background & !mask2);}
                address += self.pitch as u64;
            }
    }

    fn draw_char_rgb_16(&self, address: u64, character: u8, foreground_colour: u16, background_colour: u16)
    {
        let mut address = address;
        let char_start = character as usize * 16;
        let font_data_for_char = &FONT_8X16[char_start..char_start + 16];
        let packed_foreground: u64 = ((foreground_colour as u64) << 48) |
            ((foreground_colour as u64) << 32) |
            ((foreground_colour as u64) << 16) |
            foreground_colour as u64;
        let packed_background: u64 = ((background_colour as u64) << 48) |
            ((background_colour as u64) << 32) |
            ((background_colour as u64) << 16) |
            background_colour as u64;

        for row in 0..16
            {
                let row_data = font_data_for_char[row];
                let mask1 = LOOKUP_TABLE_16BIT[(row_data >> 4) as usize];
                let mask2 = LOOKUP_TABLE_16BIT[(row_data & 0x0F) as usize];
                unsafe {*(address as *mut u64) = (packed_foreground & mask1) | (packed_background & !mask1);}
                unsafe {*((address + 8) as *mut u64) = (packed_foreground & mask2) | (packed_background & !mask2);}
                address += self.pitch as u64;
            }


    }

    fn draw_char_rgb_8(&self, address: u64, character: u8, foreground_colour: u8, background_colour: u8)
    {
        let mut address = address;
        let char_start = character as usize * 16;
        let font_data_for_char = &FONT_8X16[char_start..char_start + 16];
        let packed_foreground: u32 = ((foreground_colour as u32) << 24) |
            ((foreground_colour as u32) << 16) |
            ((foreground_colour as u32) << 8) |
            foreground_colour as u32;
        let packed_background: u32 = ((background_colour as u32) << 24) |
            ((background_colour as u32) << 16) |
            ((background_colour as u32) << 8) |
            background_colour as u32;

        for row in 0..16
            {
                let row_data = font_data_for_char[row];
                let mask1 = LOOKUP_TABLE_8BIT[(row_data >> 4) as usize];
                let mask2 = LOOKUP_TABLE_8BIT[(row_data & 0x0F) as usize];
                unsafe {*(address as *mut u32) = (packed_foreground & mask1) | (packed_background & !mask1);}
                unsafe {*((address + 4) as *mut u32) = (packed_foreground & mask2) | (packed_background & !mask2);}
                address += self.pitch as u64;
            }
    }

    fn draw_char_ega(&self, row: u32, col: u32, character: u8)
    {
        let address = self.addr as *mut ScreenChar;
        unsafe {*address.offset((row * self.width + col) as isize) = ScreenChar
            {
                ascii_character: character,
                color_code: self.color.color_code
            };}
    }

}

use core::fmt;
impl fmt::Write for Writer
{
    fn write_str(&mut self, string: &str) -> fmt::Result
    {
        for byte in string.bytes()
            {
                self.write_byte(byte)
            }
        Ok(())
    }
}

pub fn convert_color_to_16(color: u32) -> u16
{
    let r = ((color & 0xff) >> 3) as u16;
    let g = ((color & 0xff00) >> 11) as u16;
    let b = ((color & 0xff0000) >> 19) as u16;
    let a = (color >> 31) as u16;
    (a << 15) | (b << 10) | (g << 5) | (r)
}

pub fn convert_color_to_8(color: u32) -> u8
{
    let r = ((color & 0xff) >> 5) as u8;
    let g = ((color & 0xff00) >> 13) as u8;
    let b = ((color & 0xff0000) >> 22) as u8;
    (r << 5) | (g << 2) | (b)
}