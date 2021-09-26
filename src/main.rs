#![feature(core_intrinsics)]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use x86_64::instructions::hlt;

mod color;
mod serial;
mod testable;
mod utils;
mod vga_buffer;

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        use testable::Testable;
        test.run();
    }
    utils::exit_qemu(utils::QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    utils::exit_qemu(utils::QemuExitCode::Failed);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    utils::print_logo();
    println!("");
    println!("Welcome!");
    println!("Awruk!");
    println!("Test value: {}\n", 5132.31);

    #[cfg(test)]
    test_main();
    // panic!("help!");
    loop {
        hlt();
    }
}
