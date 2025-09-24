#![no_std]
#![no_main]

use core::panic::PanicInfo;
use ren_os::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!! -> {}", info);
    loop {

    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello, World!");

    ren_os::init(); // init

    x86_64::instructions::interrupts::int3();

    println!("got to this point!\n");

    loop {}
}
