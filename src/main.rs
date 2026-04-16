#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(xiangqios::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use xiangqios::println;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Xiangqi goated{}", "!");
    xiangqios::init();
    
    //x86_64::instructions::interrupts::int3();

    /*fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();
*/
    use x86_64::registers::control::Cr3;

    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    let ptr = 0x2050f6 as *mut u8;
    unsafe { let x = *ptr; }
    println!("Read worked!");

    unsafe { *ptr = 42; }
    println!("write worked!");

    #[cfg(test)]
    test_main();
    
    println!("It did not crash!");
    xiangqios::hlt_loop();

}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    xiangqios::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    xiangqios::test_panic_handler(info)
}
