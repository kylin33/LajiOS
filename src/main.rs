#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(LajiOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use LajiOS::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    LajiOS::test_panic_handler(info)
}


#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    LajiOS::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {
        use LajiOS::print;
        print!("-");
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
