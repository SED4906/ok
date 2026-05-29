use core::fmt;
use spin::Mutex;

pub struct Writer {}
static WRITER: Mutex<Writer> = Mutex::new(Writer {});
pub static mut COL: usize = 0;
pub static mut ROW: usize = 0;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let framebuffer = crate::gfx::framebuffer::FRAMEBUFFER.lock();
        if let Some(framebuffer) = &*framebuffer {
            for c in s.as_bytes() {
                unsafe {
                    match c {
                        8 => {
                            COL = COL.saturating_sub(1);
                        }
                        9 => {
                            COL += 8;
                            if COL >= framebuffer.width / 8 {
                                COL = 0;
                                ROW += 1;
                                if ROW >= framebuffer.height / 8 {
                                    ROW = 0;
                                }
                            }
                        }
                        13 => {
                            COL = 0;
                        }
                        10 => {
                            COL = 0;
                            ROW += 1;
                            if ROW >= framebuffer.height / 8 {
                                ROW = 0;
                            }
                        }
                        _ => {
                            framebuffer.rect(
                                COL * 8,
                                ROW * 8,
                                COL * 8 + 8,
                                ROW * 8 + 8,
                                0x00000000,
                                0x00000000,
                            );
                            framebuffer.character(COL * 8, ROW * 8, *c, 0xFFFFFFFF);
                            COL += 1;
                            if COL >= framebuffer.width / 8 {
                                COL = 0;
                                ROW += 1;
                                if ROW >= framebuffer.height / 8 {
                                    ROW = 0;
                                }
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
