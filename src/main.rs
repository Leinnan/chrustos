#![feature(core_intrinsics)]
#![feature(lang_items)]
#![no_std]
#![no_main]

use core::fmt::Write;
use core::panic::PanicInfo;

use x86_64::instructions::hlt;

mod color;
mod cursor;
mod vga_buffer;
use color::{Color, ColorCode};
use cursor::Cursor;
use vga_buffer::*;

pub fn print_something() {
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!\n");
    writer.write_string("Awruk!\n");
}

#[panic_handler]
#[no_mangle]
pub fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {
        hlt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_something();
    println!("DSADSA");
    println!("Test {}", 5132.31);
    // panic!("help!");
    loop {
        hlt();
    }
}
