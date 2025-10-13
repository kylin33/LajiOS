#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(LajiOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use LajiOS::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    LajiOS::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    LajiOS::test_panic_handler(info)
}


fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use LajiOS::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    LajiOS::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    LajiOS::hlt_loop();
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
