use crate::{debug_println, print, return_if};
use alloc::string::String;
use alloc::{collections::BTreeMap, string::ToString};
use core::alloc::Layout;
use core::ptr::addr_of_mut;

static mut C_ALLOCATIONS: BTreeMap<*mut u8, Layout> = BTreeMap::new();

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, 1).unwrap();
        let allocation = alloc::alloc::alloc(layout);
        C_ALLOCATIONS.insert(allocation, layout);
        allocation
    }
}

#[no_mangle]
pub extern "C" fn calloc(items: usize, size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(items * size, 1).unwrap();
        let allocation = alloc::alloc::alloc_zeroed(layout);
        C_ALLOCATIONS.insert(allocation, layout);
        allocation
    }
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }
    unsafe { alloc::alloc::dealloc(ptr, *C_ALLOCATIONS.get(&ptr).unwrap()) }
}

#[no_mangle]
pub extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, 1).unwrap();
        let allocation = if ptr.is_null() {
            alloc::alloc::alloc_zeroed(layout)
        } else {
            alloc::alloc::realloc(ptr, *C_ALLOCATIONS.get(&ptr).unwrap(), size)
        };
        C_ALLOCATIONS.insert(allocation, layout);
        allocation
    }
}

#[no_mangle]
pub extern "C" fn strcmp(s1: *mut u8, s2: *mut u8) -> isize {
    unsafe {
        let mut index = 0;
        while *s1.byte_add(index) == *s2.byte_add(index)
            && *s1.byte_add(index) != 0
            && *s2.byte_add(index) != 0
        {
            index += 1;
        }
        (*s1.byte_add(index) as isize) - (*s2.byte_add(index) as isize)
    }
}

#[no_mangle]
pub extern "C" fn __vsnprintf_chk() {}

#[no_mangle]
extern "C" fn __stack_chk_fail() -> ! {
    panic!("stack check fail");
}

#[no_mangle]
extern "C" fn copysign(mag: f64, sgn: f64) -> f64 {
    unsafe { core::intrinsics::copysignf64(mag, sgn) }
}

#[no_mangle]
extern "C" fn copysignf(mag: f32, sgn: f32) -> f32 {
    unsafe { core::intrinsics::copysignf32(mag, sgn) }
}

#[no_mangle]
extern "C" fn floor(val: f64) -> f64 {
    unsafe { core::intrinsics::floorf64(val) }
}

#[no_mangle]
extern "C" fn floorf(val: f32) -> f32 {
    unsafe { core::intrinsics::floorf32(val) }
}

#[no_mangle]
extern "C" fn ceil(val: f64) -> f64 {
    unsafe { core::intrinsics::ceilf64(val) }
}

#[no_mangle]
extern "C" fn ceilf(val: f32) -> f32 {
    unsafe { core::intrinsics::ceilf32(val) }
}

#[no_mangle]
extern "C" fn sqrt(val: f64) -> f64 {
    unsafe { core::intrinsics::sqrtf64(val) }
}

#[no_mangle]
extern "C" fn sqrtf(val: f32) -> f32 {
    unsafe { core::intrinsics::sqrtf32(val) }
}

#[no_mangle]
extern "C" fn trunc(val: f64) -> f64 {
    unsafe { core::intrinsics::truncf64(val) }
}

#[no_mangle]
extern "C" fn truncf(val: f32) -> f32 {
    unsafe { core::intrinsics::truncf32(val) }
}

#[no_mangle]
extern "C" fn rint(val: f64) -> f64 {
    unsafe { core::intrinsics::rintf64(val) }
}

#[no_mangle]
extern "C" fn rintf(val: f32) -> f32 {
    unsafe { core::intrinsics::rintf32(val) }
}

static mut ERRNO: i32 = 0;

#[no_mangle]
extern "C" fn __errno_location() -> *mut i32 {
    unsafe { addr_of_mut!(ERRNO) }
}

fn cstr_len(ptr: *const u8) -> usize {
    let mut index = 0;
    while unsafe { *ptr.byte_add(index) } != 0 {
        index += 1;
    }
    index
}

#[no_mangle]
extern "C" fn open(pathname: *const u8, flags: i32, _mode: i32) -> i32 {
    debug_println!("(open)");
    let name = String::from_utf8_lossy(unsafe {
        core::slice::from_raw_parts(pathname, cstr_len(pathname))
    })
    .to_string();
    crate::fs::open(
        name,
        flags & 0o2000 != 0,
        flags & 0o200 != 0,
        flags & 0o1000 != 0,
    ) as i32
}

#[no_mangle]
extern "C" fn close(file_descriptor: i32) -> i32 {
    debug_println!("(close)");
    crate::fs::close(file_descriptor as isize);
    0
}

#[no_mangle]
extern "C" fn fcntl(_fd: i32, _cmd: i32, _arg: i32) -> i32 {
    debug_println!("(fcntl)");
    0
}

#[no_mangle]
extern "C" fn fstat(_fd: i32, _buf: *mut u8) -> i32 {
    debug_println!("(fstat)");
    0
}

#[no_mangle]
extern "C" fn lseek(fd: i32, offset: i64, whence: i32) -> i64 {
    debug_println!("(lseek)");
    crate::fs::seek(fd as isize, offset as isize, whence) as i64
}

#[no_mangle]
extern "C" fn openat(_dirfd: i32, _pathname: *const u8, _flags: i32, _mode: i32) -> i32 {
    debug_println!("(openat)");
    -1
}

#[repr(C)]
struct IOVector {
    base: *mut u8,
    size: usize,
}

#[no_mangle]
extern "C" fn readv(fd: i32, bufs: *mut IOVector, bufcnt: i32) -> i64 {
    debug_println!("(readv)");
    match fd {
        1 | 2 => 0,
        0 => 0,
        handle => {
            let mut count = 0;
            unsafe {
                let iovecs = core::slice::from_raw_parts(bufs, bufcnt as usize);
                for iovec in iovecs {
                    let slice = core::slice::from_raw_parts_mut(iovec.base, iovec.size);
                    let wasread = crate::fs::read(handle as isize, slice);
                    return_if!(wasread < 0, wasread as i64);
                    count += wasread;
                }
            }
            count as i64
        }
    }
}

#[no_mangle]
extern "C" fn writev(fd: i32, bufs: *mut IOVector, bufcnt: i32) -> i64 {
    debug_println!("(writev)");
    match fd {
        1 | 2 => {
            let mut count = 0;
            unsafe {
                let iovecs = core::slice::from_raw_parts(bufs, bufcnt as usize);
                for iovec in iovecs {
                    let slice = core::slice::from_raw_parts(iovec.base, iovec.size);
                    print!("{}", String::from_utf8_lossy(slice));
                    count += iovec.size;
                }
            }
            count as i64
        }
        0 => 0,
        handle => {
            let mut count = 0;
            unsafe {
                let iovecs = core::slice::from_raw_parts(bufs, bufcnt as usize);
                for iovec in iovecs {
                    let slice = core::slice::from_raw_parts(iovec.base, iovec.size);
                    let written = crate::fs::write(handle as isize, slice);
                    return_if!(written < 0, written as i64);
                    count += written;
                }
            }
            count as i64
        }
    }
}

#[no_mangle]
extern "C" fn fdatasync(_fd: i32) -> i32 {
    debug_println!("(fdatasync)");
    0
}

#[no_mangle]
extern "C" fn getrandom() -> u64 {
    debug_println!("(getrandom)");
    let mut random_value: u64 = 0;
    unsafe {
        x86::random::rdrand64(&mut random_value);
    }
    random_value
}

#[no_mangle]
extern "C" fn clock_getres(_clockid: i32, _res: *mut u8) -> i32 {
    debug_println!("(clock_getres)");
    -1
}

#[no_mangle]
extern "C" fn clock_gettime(_clockid: i32, _tp: *mut u8) -> i32 {
    debug_println!("(clock_gettime)");
    -1
}
