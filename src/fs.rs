use crate::gfx::framebuffer::FRAMEBUFFER;
use crate::return_if;
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
static NEXT_HANDLE: Mutex<isize> = Mutex::new(5);

//pub fn fs_init() {}

pub struct OpenFlags {
    pub append: bool,
    pub exclude: bool,
    pub truncate: bool,
}

pub fn preload(name: String, data: Vec<u8>) {
    let mut file_system = FILE_SYSTEM.lock();
    let _ = file_system.insert(name, data);
}

pub fn exists(name: &str) -> bool {
    FILE_SYSTEM.lock().contains_key(name)
}

pub fn size(name: &str) -> usize {
    match FILE_SYSTEM.lock().get(name) {
        Some(data) => data.len(),
        None => 0,
    }
}

pub fn hsize(handle: isize) -> usize {
    match HANDLES.lock().get(&handle) {
        Some(o) => o.data.len(),
        None => 0,
    }
}

pub fn path(handle: isize) -> String {
    match HANDLES.lock().get(&handle) {
        Some(o) => o.path.clone(),
        None => "".into(),
    }
}

pub fn open(name: String, open_flags: OpenFlags) -> isize {
    if name == "./palette.raw" {
        return 7;
    } else if name == "./screen.data" {
        return 6;
    }
    let mut next_handle = NEXT_HANDLE.lock();
    let mut file_system = FILE_SYSTEM.lock();
    let mut handles = HANDLES.lock();
    return_if!(open_flags.exclude && file_system.get(&name).is_some(), -1);
    let handle = *next_handle;
    *next_handle += 1;
    if handle == 5 {
        *next_handle = 8;
    }
    handles.insert(
        handle,
        OpenFile {
            position: if open_flags.append && !open_flags.truncate {
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

pub fn valid(handle: isize) -> bool {
    handle == 3 || HANDLES.lock().contains_key(&handle)
}

pub fn close(handle: isize) {
    let mut file_system = FILE_SYSTEM.lock();
    let mut handles = HANDLES.lock();
    if !handles.contains_key(&handle) {
        return;
    }
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

static DOOM_FB: Mutex<[u8; 320 * 200]> = Mutex::new([0; 320 * 200]);
static PALETTE: Mutex<[u8; 256 * 3]> = Mutex::new([0; 256 * 3]);

pub fn write(handle: isize, bytes: &[u8]) -> isize {
    let mut handles = HANDLES.lock();
    if handle == 6 {
        let mut doom_fb = DOOM_FB.lock();
        let framebuffer = FRAMEBUFFER.lock();
        let palette = PALETTE.lock();
        doom_fb[0..bytes.len()].copy_from_slice(bytes);
        let mut pos = 0;
        if let Some(fb) = *framebuffer {
            let xw = fb.width / 320;
            let yh = fb.height / 200;
            for byte in bytes {
                let x = pos % 320;
                let y = pos / 320;
                let color = 0xff000000
                    | palette[*byte as usize * 3 + 2] as u32
                    | ((palette[*byte as usize * 3 + 1] as u32) << 8)
                    | ((palette[*byte as usize * 3] as u32) << 16);
                fb.rect(x * xw, y * yh, x * xw + xw, y * yh + yh, color, color);
                pos += 1;
            }
        }
        return bytes.len() as isize;
    }
    if handle == 7 {
        let mut palette = PALETTE.lock();
        palette[0..bytes.len()].copy_from_slice(bytes);
        return bytes.len() as isize;
    }
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
                let data_len = open_file.data.len();
                let last_position = open_file.position + bytes.len();
                let middle_position = last_position - data_len;
                open_file
                    .data
                    .get_mut(open_file.position..data_len)
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
            if open_file.position < open_file.data.len() {
                let data_len = open_file.data.len();
                let length = data_len.min(bytes.len());
                bytes.get_mut(0..length).unwrap().clone_from_slice(
                    open_file
                        .data
                        .get_mut(open_file.position..open_file.position + length)
                        .unwrap(),
                );
                open_file.position += length;
                length as isize
            } else {
                0
            }
        }
        None => -1,
    }
}
