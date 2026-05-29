#![feature(abi_x86_interrupt, pointer_is_aligned_to)]
#![no_std]
#![no_main]

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/cpu.rs")]
mod cpu;
mod fs;
mod gfx;
mod helper;
mod irq;
mod mm;
#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/serial.rs")]
mod serial;
mod wasi;

extern crate alloc;

use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::panic::PanicInfo;
use wasmi::{Engine, Linker, Module, Store};

static MODULE_REQUEST: limine::request::ModulesRequest = limine::request::ModulesRequest::new();
static CMDLINE_REQUEST: limine::request::ExecutableCmdlineRequest =
    limine::request::ExecutableCmdlineRequest::new();

#[unsafe(no_mangle)]
extern "C" fn _start() -> ! {
    serial::serial_init();
    println!("ok");
    gfx::framebuffer::framebuffer_init();
    gprintln!("ok");
    mm::arch::mm_init();
    gprintln!("mm");
    irq::arch::irq_init();
    gprintln!("irq");
    cpu::cpu_init();
    gprintln!("cpu");
    let mut code_bytes = include_bytes!("wasm_print.wasm").to_vec();
    let mut code_envs = vec![];
    let mut code_args = vec![];
    if let Some(response) = MODULE_REQUEST.response() {
        for module in response.modules() {
            if module.cmdline().is_empty() {
                code_bytes = module.data().to_vec();
            } else if module.cmdline() == "/etc/profile" {
                for line in String::from_utf8_lossy(module.data()).lines() {
                    if let Some(_) = line.split_once('=') {
                        code_envs.push(line.to_owned());
                    }
                }
            } else {
                fs::preload(module.cmdline().into(), module.data().to_vec());
            }
        }
    }
    if let Some(response) = CMDLINE_REQUEST.response() {
        code_args.append(
            &mut response
                .cmdline()
                .split_ascii_whitespace()
                .map(|s| s.to_string())
                .collect(),
        );
    } else {
        code_args.push("init".to_string());
    }
    // Language runtime below
    let engine = Engine::default();
    let module = Module::new(&engine, code_bytes).expect("Unable to parse wasm module");
    let mut store = Store::new(&engine, (code_args, code_envs));
    let mut linker = <Linker<(Vec<String>, Vec<String>)>>::new(&engine);
    wasi::link_wasi(&mut linker);
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
    gprintln!();
    gprintln!("                             ");
    gprintln!("                             ");
    gprintln!("    FLAGRANT SYSTEM ERROR    ");
    gprintln!("       Computer over.        ");
    gprintln!("      Panic = Very Yes.      ");
    gprintln!("                             ");
    gprintln!("                             ");
    gprintln!("                             ");
    gprintln!("{info}");
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
