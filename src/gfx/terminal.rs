use core::{
    fmt,
    sync::atomic::{AtomicUsize, Ordering},
};
use spin::Mutex;

pub struct Writer {}
static WRITER: Mutex<Writer> = Mutex::new(Writer {});
pub static COL: AtomicUsize = AtomicUsize::new(0);
pub static ROW: AtomicUsize = AtomicUsize::new(0);

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let framebuffer = crate::gfx::framebuffer::FRAMEBUFFER.lock();
        if let Some(framebuffer) = &*framebuffer {
            for c in s.as_bytes() {
                match c {
                    8 => {
                        COL.store(
                            COL.load(Ordering::Relaxed).saturating_sub(1),
                            Ordering::Relaxed,
                        );
                    }
                    9 => {
                        COL.store(
                            COL.load(Ordering::Relaxed) + 8 - (COL.load(Ordering::Relaxed) % 8),
                            Ordering::Relaxed,
                        );
                        if COL.load(Ordering::Relaxed) >= framebuffer.width / 4 {
                            COL.store(0, Ordering::Relaxed);
                            ROW.fetch_add(1, Ordering::Relaxed);
                            if ROW.load(Ordering::Relaxed) >= framebuffer.height / 5 {
                                ROW.store(0, Ordering::Relaxed);
                            }
                        }
                    }
                    13 => {
                        COL.store(0, Ordering::Relaxed);
                    }
                    10 => {
                        COL.store(0, Ordering::Relaxed);
                        ROW.fetch_add(1, Ordering::Relaxed);
                        if ROW.load(Ordering::Relaxed) >= framebuffer.height / 5 {
                            ROW.store(0, Ordering::Relaxed);
                        }
                    }
                    _ => {
                        framebuffer.rect(
                            COL.load(Ordering::Relaxed) * 4,
                            ROW.load(Ordering::Relaxed) * 5,
                            COL.load(Ordering::Relaxed) * 4 + 4,
                            ROW.load(Ordering::Relaxed) * 5 + 5,
                            0x00000000,
                            0x00000000,
                        );
                        framebuffer.character(
                            COL.load(Ordering::Relaxed) * 4,
                            ROW.load(Ordering::Relaxed) * 5,
                            *c,
                            0xFFFFFFFF,
                        );
                        COL.fetch_add(1, Ordering::Relaxed);
                        if COL.load(Ordering::Relaxed) >= framebuffer.width / 5 {
                            COL.store(0, Ordering::Relaxed);
                            ROW.fetch_add(1, Ordering::Relaxed);
                            if ROW.load(Ordering::Relaxed) >= framebuffer.height / 5 {
                                ROW.store(0, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! gprint {
    ($($t:tt)*) => { $crate::gfx::terminal::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! gprintln {
    ()          => { $crate::gprint!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::gprint!("{}\n", format_args!($($t)*)) };
}
