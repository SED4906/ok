#![feature(abi_x86_interrupt, pointer_is_aligned_to)]
#![no_std]
#![no_main]

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/cpu.rs")]
mod cpu;
mod fs;
mod helper;
mod irq;
mod mm;
#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/serial.rs")]
mod serial;

extern crate alloc;

use alloc::string::String;
use core::panic::PanicInfo;
use wasmi::{Caller, Engine, Extern, Linker, Module, Store};

//static MODULE_REQUEST: ModuleRequest = ModuleRequest::new();

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    serial::serial_init();
    println!("ok");
    mm::arch::mm_init();
    println!("mm");
    irq::arch::irq_init();
    println!("irq");
    cpu::cpu_init();
    println!("cpu");
    // Language runtime below
    let engine = Engine::default();
    let module = Module::new(&engine, &include_bytes!("wasm_print.wasm")[..])
        .expect("Unable to parse wasm module");
    let mut store = Store::new(&engine, ());
    let mut linker = <Linker<()>>::new(&engine);
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_write",
            |mut caller: Caller<'_, ()>, fd: i32, iovs: i32, iovs_len: i32, nwritten: i32| {
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                let mut count = 0i32;
                for iov_index in 0..iovs_len {
                    let iov = (iovs + iov_index * 8) as usize;
                    let base =
                        u32::from_le_bytes(memory[iov..iov + 4].try_into().unwrap()) as usize;
                    let size =
                        u32::from_le_bytes(memory[iov + 4..iov + 8].try_into().unwrap()) as usize;
                    let slice = &memory[base..base + size];
                    match fd {
                        0 => {}
                        1 | 2 => {
                            print!("{}", String::from_utf8_lossy(&slice));
                            count += size as i32;
                        }
                        handle => {
                            let written = fs::write(handle as isize, &slice) as i32;
                            count += written;
                        }
                    }
                }
                memory[nwritten as usize..nwritten as usize + 4]
                    .copy_from_slice(&count.to_le_bytes());
                Ok(0)
            },
        )
        .expect("failed to wrap fd_write");
    let instance = linker
        .instantiate_and_start(&mut store, &module)
        .expect("Unable to instantiate");
    let func = instance
        .get_typed_func::<(), ()>(&store, "_start")
        .expect("Unable to find _start function");
    let _ = func.call(&mut store, ());
    // Language runtime above
    println!("done!");
    loop {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            x86::halt();
        }
    }
}

#[panic_handler]
fn rust_panic(info: &PanicInfo) -> ! {
    println!("{info}");
    hcf()
}

fn hcf() -> ! {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        x86::irq::disable();
    }
    loop {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            x86::halt();
        }
    }
}
