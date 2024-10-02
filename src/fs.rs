use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use spin::Mutex;

pub struct OpenFile {
    path: String,
    position: usize,
    data: Vec<u8>,
}

static FILE_SYSTEM: Mutex<BTreeMap<String, Vec<u8>>> = Mutex::new(BTreeMap::new());
static HANDLES: Mutex<BTreeMap<isize, OpenFile>> = Mutex::new(BTreeMap::new());
static NEXT_HANDLE: Mutex<isize> = Mutex::new(3);

//pub fn fs_init() {}

pub fn open(name: String, append: bool, exclude: bool, truncate: bool) -> isize {
    let mut next_handle = NEXT_HANDLE.lock();
    let mut file_system = FILE_SYSTEM.lock();
    let mut handles = HANDLES.lock();
    let handle = *next_handle;
    *next_handle += 1;
    if exclude && file_system.get(&name).is_some() {
        return -1;
    }
    handles.insert(
        handle,
        OpenFile {
            position: if append && !truncate {
                match file_system.get(&name) {
                    Some(data) => data.len(),
                    None => 0,
                }
            } else {
                0
            },
            data: match file_system.get_mut(&name) {
                Some(data) => data.clone(),
                None => {
                    file_system.insert(name.clone(), Vec::new());
                    Vec::new()
                }
            },
            path: name,
        },
    );
    handle
}

pub fn close(handle: isize) {
    let mut file_system = FILE_SYSTEM.lock();
    let mut handles = HANDLES.lock();
    let open_file = handles.get(&handle).unwrap();
    file_system.insert(open_file.path.clone(), open_file.data.clone());
    handles.remove(&handle);
}

pub fn seek(handle: isize, offset: isize, whence: i32) -> isize {
    let mut handles = HANDLES.lock();
    match handles.get_mut(&handle) {
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
    let mut handles = HANDLES.lock();
    match handles.get_mut(&handle) {
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
    let mut handles = HANDLES.lock();
    match handles.get_mut(&handle) {
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
