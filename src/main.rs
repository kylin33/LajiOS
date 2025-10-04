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

    // x86_64::instructions::interrupts::int3(); //触发中断
    // unsafe{
    //     *(0xdeadbeef as *mut u8) = 42;
    // };     // 使用unsafe操作 无效内存地址，必然抛出 page fault
    fn stack_overflow(){
        stack_overflow();
    }

    stack_overflow();


    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    loop {}
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
