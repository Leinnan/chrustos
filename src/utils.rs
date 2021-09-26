use crate::println;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn print_logo() {

    let text = r###"
                              _           _  __
                        _ |_ |_)    _ _|_/ \(_ 
                       (_ | || \|_|_>  |_\_/__)
"###;
    crate::vga_buffer::change_color(crate::color::Color::BrightRed);
    println!("{}", text);
    crate::vga_buffer::reset_color();
}