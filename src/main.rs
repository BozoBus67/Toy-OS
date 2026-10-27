#![no_std]
#![no_main]

mod serial;
mod vga;

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, OS!");
    println!("println works: {} + {} = {}", 2, 3, 2 + 3);

    serial_println!("Hello from serial!");
    serial_println!("serial works: {} + {} = {}", 2, 3, 2 + 3);

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("PANIC: {}", info);
    loop {}
}
