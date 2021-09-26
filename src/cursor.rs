use crate::color::Color;
use core::fmt;

pub struct Cursor {
    pub position: isize,
    pub foreground: Color,
    pub background: Color,
}

impl Cursor {
    pub fn color(&self) -> u8 {
        let fg = self.foreground as u8;
        let bg = (self.background as u8) << 4;
        fg | bg
    }

    pub fn print(&mut self, text: &[u8]) {
        let color = self.color();

        let framebuffer = 0xb8000 as *mut u8;

        for &character in text {
            unsafe {
                framebuffer.offset(self.position).write_volatile(character);
                framebuffer.offset(self.position + 1).write_volatile(color);
            }
            self.position += 2;
        }
    }
    pub fn clr(&mut self) {
        self.position = 0;
        for _ in 0..(80 * 25) {
            self.print(b" ");
        }
        self.position = 0;
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            position: 0,
            foreground: Color::Green,
            background: Color::DarkGray,
        }
    }
}

impl fmt::Write for Cursor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s.as_bytes());
        Ok(())
    }
}
