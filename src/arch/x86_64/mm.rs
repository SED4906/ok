use limine::memory_map::EntryType;
use linked_list_allocator::LockedHeap;
use x86::controlregs::cr3;

use crate::return_if;

use super::{link_page, unlink_page};

static MEMMAP_REQUEST: limine::request::MemoryMapRequest = limine::request::MemoryMapRequest::new();
const HEAP_START: u64 = 320u64 << 39;
//const HEAP_DEFAULT_SIZE: usize = 4096usize * 1024;
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// Initializes the freelist.
///
/// # Safety
///
/// Should be safe as long as what Limine reports is correct.
pub fn mm_init() {
    let entries = MEMMAP_REQUEST.get_response().unwrap().entries();
    for entry in entries {
        if entry.entry_type != EntryType::USABLE {
            continue;
        }
        free_region(entry.base, entry.length);
    }
    let mut heap_page = 0;
    loop {
        unsafe {
            let page = unlink_page::<u8>();
            if page.is_null() {
                break;
            }
            map_page(cr3(), HEAP_START + 4096 * heap_page, page as u64, 3, 4096);
        }
        heap_page += 1;
    }
    unsafe {
        HEAP.lock().init(
            ((0xffffu64 << 48) + HEAP_START) as *mut u8,
            (heap_page * 4096) as usize,
        )
    };
}

/// Frees a contiguous region of memory.
///
/// # Safety
///
/// Safety rules for `link_page(*mut _)` still apply.
fn free_region(base: u64, length: u64) {
    let mut page = base;
    while page < base + length {
        link_page(page as *mut u8);
        page += 4096;
    }
}

/// Maps a physical page to a virtual page.
///
/// # Safety
///
/// Pagemap must be a valid PML4.
pub fn map_page(pagemap: u64, v_address: u64, p_address: u64, flags: u64, size: u64) {
    match size {
        4096 => {
            let level3 = get_next_level(pagemap, v_address, 3);
            return_if!(level3 == 0);
            let level2 = get_next_level(level3, v_address, 2);
            return_if!(level2 == 0);
            let level1 = get_next_level(level2, v_address, 1);
            return_if!(level1 == 0);
            unsafe { (*(level1 as *mut [u64; 512]))[(v_address as usize >> 12) & 0x1FF] = p_address | flags; }
        }
        _ => panic!("invalid page size"),
    }
}

/// Gets an entry from a pagemap, creating one if it is not present.
///
/// # Safety
///
/// Pagemap must be valid.
pub fn get_next_level(pagemap: u64, v_address: u64, level: u64) -> u64 {
    let result = unsafe { (*(pagemap as *mut [u64; 512]))[(v_address as usize >> (12 + 9 * level)) & 0x1FF] };
    if result & 1 == 0 {
        let page = unlink_page::<[u64; 512]>();
        return_if!(page.is_null(), 0);
        unsafe {
            (*page).fill(0);
            (*(pagemap as *mut [u64; 512]))[(v_address as usize >> (12 + 9 * level)) & 0x1FF] =
                page as u64 | 7;
        }
        return page as u64;
    }
    result & !0xFFF
}
