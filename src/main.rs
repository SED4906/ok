#![allow(internal_features)]
#![feature(
    core_intrinsics,
    naked_functions,
    abi_x86_interrupt,
    pointer_is_aligned
)]
#![no_std]
#![no_main]

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/cpu.rs")]
mod cpu;
pub mod fs;
mod helper;
mod irq;
mod mm;
#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/serial.rs")]
mod serial;
mod syscall;

extern crate alloc;

use core::panic::PanicInfo;

use wasm3::Environment;
use wasm3::Module;

//static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[no_mangle]
extern "C" fn _start() -> ! {
    unsafe {
        serial::serial_init();
    }
    println!("ok");
    mm::arch::mm_init();
    println!("mm");
    irq::arch::irq_init();
    println!("irq");
    cpu::cpu_init();
    println!("cpu");
    let env = Environment::new().expect("Unable to create environment");
    let rt = env
        .create_runtime(1024 * 64)
        .expect("Unable to create runtime");
    let module = Module::parse(&env, &include_bytes!("wasm_print.wasm")[..])
        .expect("Unable to parse module");
    let mut module = rt.load_module(module).expect("Unable to load module");
    module.link_wasi().expect("Failed to link wasi");
    let func = module
        .find_function::<(), ()>("_start")
        .expect("Unable to find function");
    func.call().unwrap();
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
