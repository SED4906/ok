#![feature(naked_functions, abi_x86_interrupt, pointer_is_aligned)]
#![no_std]
#![no_main]

mod helper;
mod irq;
mod mm;
#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/serial.rs")]
mod serial;

extern crate alloc;

use core::panic::PanicInfo;

#[no_mangle]
extern "C" fn _start() -> ! {
    #[cfg(debug_assertions)]
    unsafe {
        serial::serial_init();
    }
    println!("ok");
    mm::arch::mm_init();
    println!("mm");
    irq::arch::irq_init();
    println!("irq");
    println!("done!");
    loop {
        unsafe {
            x86::halt();
        }
    }
}

#[panic_handler]
unsafe fn rust_panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hcf()
}

unsafe fn hcf() -> ! {
    x86::irq::disable();
    loop {
        x86::halt();
    }
}
