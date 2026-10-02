use linked_list_allocator::LockedHeap;
use x86::controlregs::cr3;

use crate::println;

use crate::mm::{link_page, unlink_page};

static MEMMAP_REQUEST: limine::request::MemmapRequest = limine::request::MemmapRequest::new();
const HEAP_START: u64 = 320u64 << 39;
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

pub const PAGE_SIZE: u64 = 4096;

pub fn mm_init() {
    let entries = MEMMAP_REQUEST.response().unwrap().entries();
    for entry in entries {
        if entry.type_ != limine::memmap::MEMMAP_USABLE {
            continue;
        }
        free_region(entry.base, entry.length);
    }
    let mut heap_page = 0;
    loop {
            let page = unlink_page::<u8>();
            if page.is_null()
                || map_page(unsafe{cr3()}, HEAP_START + PAGE_SIZE * heap_page, page as u64, 3).is_none()
            {
                break;
            }

        heap_page += 1;
    }
    unsafe {
        HEAP.lock().init(
            ((0xffffu64 << 48) + HEAP_START) as *mut u8,
            (heap_page * PAGE_SIZE) as usize,
        )
    };
    println!("{} KiB heap space", heap_page * 4);
}

fn free_region(base: u64, length: u64) {
    let mut page = base;
    while page < base + length {
        link_page(page as *mut u8);
        page += PAGE_SIZE;
    }
}

pub fn map_page(pagemap: u64, v_address: u64, p_address: u64, flags: u64) -> Option<()> {
    let level3 = get_next_level(pagemap, v_address, 3)?;
    let level2 = get_next_level(level3, v_address, 2)?;
    let level1 = get_next_level(level2, v_address, 1)?;
    unsafe {
        (*(level1 as *mut [u64; 512]))[(v_address as usize >> 12) & 0x1FF] = p_address | flags;
    }
    Some(())
}

pub fn get_next_level(pagemap: u64, v_address: u64, level: u64) -> Option<u64> {
    let result = unsafe {
        (*(pagemap as *mut [u64; 512]))[(v_address as usize >> (12 + 9 * level)) & 0x1FF]
    };
    if result & 1 == 0 {
        let page = unlink_page::<[u64; 512]>();
        if page.is_null() {
            return None;
        }
        unsafe {
            (*page).fill(0);
            (*(pagemap as *mut [u64; 512]))[(v_address as usize >> (12 + 9 * level)) & 0x1FF] =
                page as u64 | 7;
        }
        return Some(page as u64);
    }
    Some(result & !0xFFF)
}
