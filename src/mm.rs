#[cfg(target_arch = "x86_64")]
const PAGE_SIZE: usize = 4096;

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/mm.rs")]
pub mod arch;

use core::{
    ptr::null_mut,
    sync::atomic::{AtomicPtr, Ordering},
};

static FREELIST: AtomicPtr<()> = AtomicPtr::new(null_mut());

pub fn link_page<T>(page: *mut T) {
    assert!(!page.is_null() && page.addr().is_multiple_of(PAGE_SIZE));
    unsafe {
        *page.cast() = FREELIST.swap(page.cast(), Ordering::Relaxed);
    }
}

pub fn unlink_page<T>() -> *mut T {
    FREELIST.update(Ordering::SeqCst, Ordering::SeqCst, |p| {
        if !p.is_null() {
            return unsafe { *p.cast() };
        }
        p
    }).cast()
}
