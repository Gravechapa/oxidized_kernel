mod font_8x16;

use self::font_8x16::*;

pub fn draw_char(address: u64, pitch: u32, character: u8, foreground_colour: u32, background_colour: u32)
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
        address += pitch as u64;
    }
}