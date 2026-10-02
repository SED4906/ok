pub mod irq;
pub mod mm;
pub mod serial;
pub mod timer;

pub fn wfe() -> ! {
    loop {
        unsafe {
            x86::halt();
        }
    }
}

pub fn hcf() -> ! {
    unsafe {
        x86::irq::disable();
        loop {
            x86::halt();
        }
    }
}
