use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

pub struct OpenFile {
    path: String,
    position: usize,
    data: Vec<u8>,
}

static mut FILE_SYSTEM: BTreeMap<String, Vec<u8>> = BTreeMap::new();
static mut HANDLES: BTreeMap<isize, OpenFile> = BTreeMap::new();
static mut NEXT_HANDLE: isize = 3;

pub fn fs_init() {}

pub fn open(name: String, append: bool, exclude: bool, truncate: bool) -> isize {
    unsafe {
        let handle = NEXT_HANDLE;
        NEXT_HANDLE += 1;
        if exclude && FILE_SYSTEM.get(&name).is_some() {
            return -1;
        }
        HANDLES.insert(
            handle,
            OpenFile {
                position: if append && !truncate {
                    match FILE_SYSTEM.get(&name) {
                        Some(data) => data.len(),
                        None => 0,
                    }
                } else {
                    0
                },
                data: match FILE_SYSTEM.get_mut(&name) {
                    Some(data) => data.clone(),
                    None => {
                        FILE_SYSTEM.insert(name.clone(), Vec::new());
                        Vec::new()
                    }
                },
                path: name,
            },
        );
        handle
    }
}

pub fn close(handle: isize) {
    unsafe {
        let open_file = HANDLES.get(&handle).unwrap();
        FILE_SYSTEM.insert(open_file.path.clone(), open_file.data.clone());
        HANDLES.remove(&handle);
    }
}

pub fn seek(handle: isize, offset: isize, whence: i32) -> isize {
    match unsafe { HANDLES.get_mut(&handle) } {
        Some(open_file) => {
            match whence {
                0 => open_file.position = offset.max(0) as usize,
                1 => open_file.position = open_file.position.saturating_add_signed(offset),
                2 => open_file.position = open_file.data.len().saturating_add_signed(offset),
                _ => return -1,
            };
            open_file.position as isize
        }
        None => -1,
    }
}

pub fn write(handle: isize, bytes: &[u8]) -> isize {
    match unsafe { HANDLES.get_mut(&handle) } {
        Some(open_file) => {
            if open_file.position + bytes.len() <= open_file.data.len() {
                let last_position = open_file.position + bytes.len();
                open_file
                    .data
                    .get_mut(open_file.position..last_position)
                    .unwrap()
                    .clone_from_slice(bytes);
            } else if open_file.position > open_file.data.len() {
                open_file
                    .data
                    .append(&mut [0].repeat(open_file.position - open_file.data.len()));
                open_file.data.append(&mut bytes.to_vec().clone());
            } else {
                let last_position = open_file.position + bytes.len();
                let middle_position = open_file.data.len() - open_file.position;
                open_file
                    .data
                    .get_mut(open_file.position..last_position)
                    .unwrap()
                    .clone_from_slice(bytes.get(0..middle_position).unwrap());
                open_file.data.append(
                    &mut bytes
                        .get(middle_position..bytes.len())
                        .unwrap()
                        .to_vec()
                        .clone(),
                );
            }
            open_file.position += bytes.len();
            bytes.len() as isize
        }
        None => -1,
    }
}

pub fn read(handle: isize, bytes: &mut [u8]) -> isize {
    match unsafe { HANDLES.get_mut(&handle) } {
        Some(open_file) => {
            if open_file.position + bytes.len() > open_file.data.len() {
                let data_len = open_file.data.len();
                let length = data_len - bytes.len();
                bytes.get_mut(0..length).unwrap().clone_from_slice(
                    open_file
                        .data
                        .get_mut(open_file.position..data_len)
                        .unwrap(),
                );
                open_file.position = data_len;
                length as isize
            } else if open_file.position >= open_file.data.len() {
                0
            } else {
                let last_position = open_file.position + bytes.len();
                bytes.clone_from_slice(
                    open_file
                        .data
                        .get_mut(open_file.position..last_position)
                        .unwrap(),
                );
                open_file.position += bytes.len();
                bytes.len() as isize
            }
        }
        None => -1,
    }
}
