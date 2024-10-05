use core::ptr::null_mut;

use crate::return_if;

pub struct Freelist(*mut Freelist);
static mut FREELIST: *mut Freelist = null_mut();

#[cfg(target_arch = "x86_64")]
const PAGE_SIZE: usize = 4096;

#[cfg_attr(target_arch = "x86_64", path = "arch/x86_64/mm.rs")]
pub mod arch;

/// Links a page to the freelist.
///
/// # Safety
///
/// The memory must not already be in use.
pub unsafe fn link_page<T>(address: *mut T) {
    #[cfg(target_arch = "x86_64")]
    return_if!(!address.is_aligned_to(PAGE_SIZE));
    let page = address as *mut Freelist;
    (*page).0 = FREELIST;
    FREELIST = page;
}

/// Unlinks a page from the freelist.
///
/// # Safety
///
/// Returns `null_mut()` if the freelist runs out of memory.
pub unsafe fn unlink_page<T>() -> *mut T {
    return_if!(FREELIST.is_null(), null_mut());
    let address = FREELIST as *mut _;
    FREELIST = (*FREELIST).0;
    address
}
