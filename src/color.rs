#[allow(unused)]
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Color {
    Black = 0x0,
    White = 0xF,
    Blue = 0x1,
    BrightBlue = 0x9,
    Green = 0x2,
    BrightGreen = 0xA,
    Cyan = 0x3,
    BrightCyan = 0xB,
    Red = 0x4,
    BrightRed = 0xC,
    Magenta = 0x5,
    BrightMagenta = 0xD,
    Brown = 0x6,
    Yellow = 0xE,
    Gray = 0x7,
    DarkGray = 0x8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}
