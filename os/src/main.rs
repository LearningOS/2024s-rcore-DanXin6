#![no_std]
#![no_main]
#![feature(panic_info_message)]


use core::arch::global_asm;

mod lang_items;
mod sbi;
#[macro_use]
mod console;

global_asm!(include_str!("entry.asm"));

// Clear the .bss section
fn clear_bss() {
    extern "C" {
        fn sbss(); // Start of the .bss section
        fn ebss(); // End of the .bss section
    }

    unsafe {
        (sbss as *mut u8).write_bytes(
            0,
            (ebss as usize - sbss as usize) / core::mem::size_of::<u8>(),
        )
    };
}

#[no_mangle]
fn rust_main() {
    clear_bss();

    println!("[rCore] Hello, world!");
    loop {}
}
