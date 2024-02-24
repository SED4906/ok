use alloc::collections::BTreeMap;
use alloc::string::String;

pub struct OpenFile<'a> {
    position: usize,
    data: &'a mut [u8],
}

static mut FILE_SYSTEM: BTreeMap<String, &mut [u8]> = BTreeMap::new();
static mut HANDLES: BTreeMap<isize, OpenFile> = BTreeMap::new();
static mut NEXT_HANDLE: isize = 3;

pub fn fs_init() {
}

pub fn open(name: String, append: bool, exclude: bool, truncate: bool) -> isize {
    unsafe {
        let handle = NEXT_HANDLE;
        NEXT_HANDLE += 1;
        if exclude && FILE_SYSTEM.get(&name).is_some() {
            return -1;
        }
        HANDLES.insert(handle, OpenFile {
            position: if append && !truncate {
                match FILE_SYSTEM.get(&name) {
                    Some(data) => data.len(),
                    None => 0,
                }
            } else {0},
            data: match FILE_SYSTEM.get_mut(&name) {
                Some(data) => *data,
                None => {
                    FILE_SYSTEM.insert(name.clone(), &mut []);
                    *FILE_SYSTEM.get_mut(&name).unwrap()
                }
            },
        });
        handle
    }
}