use alloc::{string::String, vec::Vec};
use wasmi::{Caller, Extern, Linker};

use crate::{
    debug_println,
    fs::{self, OpenFlags, valid},
    gprint, println,
};

pub fn link_wasi(linker: &mut Linker<(Vec<String>, Vec<String>)>) {
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "args_sizes_get",
            |mut caller: Caller<'_, (Vec<String>, _)>, num: i32, buf_size: i32| {
                debug_println!("args_sizes_get({num}, {buf_size})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, (args, _)) = memory.data_and_store_mut(&mut caller);
                memory[num as usize..num as usize + 4]
                    .copy_from_slice(&(args.len() as u32).to_le_bytes());
                let mut size = 1;
                for arg in args {
                    size += arg.len() + 1;
                }
                memory[buf_size as usize..buf_size as usize + 4]
                    .copy_from_slice(&(size as u32).to_le_bytes());
                Ok(0)
            },
        )
        .expect("failed to wrap args_sizes_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "args_get",
            |mut caller: Caller<'_, (Vec<String>, _)>, argv: i32, buf: i32| {
                debug_println!("args_get({argv}, {buf})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, (args, _)) = memory.data_and_store_mut(&mut caller);
                let mut off = 0i32;
                for arg in args {
                    memory[argv as usize..argv as usize + 4]
                        .copy_from_slice(&(buf + off).to_le_bytes());
                    memory[buf as usize + off as usize..buf as usize + arg.len() + off as usize]
                        .copy_from_slice(arg.as_bytes());
                    memory[buf as usize + arg.len() + off as usize] = 0;
                    off += (1 + arg.len()) as i32;
                }
                memory[buf as usize + off as usize] = 0;
                Ok(0)
            },
        )
        .expect("failed to wrap args_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "environ_sizes_get",
            |mut caller: Caller<'_, (_, Vec<String>)>, num: i32, buf_size: i32| {
                debug_println!("environ_sizes_get({num}, {buf_size})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, (_, envs)) = memory.data_and_store_mut(&mut caller);
                memory[num as usize..num as usize + 4]
                    .copy_from_slice(&(envs.len() as u32).to_le_bytes());
                let mut size = 1;
                for env in envs {
                    size += env.len() + 1;
                }
                memory[buf_size as usize..buf_size as usize + 4]
                    .copy_from_slice(&(size as u32).to_le_bytes());
                Ok(0)
            },
        )
        .expect("failed to wrap environ_sizes_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "environ_get",
            |mut caller: Caller<'_, (_, Vec<String>)>, envs: i32, buf: i32| {
                debug_println!("args_get({envs}, {buf})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, (_, envvars)) = memory.data_and_store_mut(&mut caller);
                let mut off = 0i32;
                for env in envvars {
                    memory[envs as usize..envs as usize + 4]
                        .copy_from_slice(&(buf + off).to_le_bytes());
                    memory[buf as usize + off as usize..buf as usize + env.len() + off as usize]
                        .copy_from_slice(env.as_bytes());
                    memory[buf as usize + env.len() + off as usize] = 0;
                    off += (1 + env.len()) as i32;
                }
                memory[buf as usize + off as usize] = 0;
                Ok(0)
            },
        )
        .expect("failed to wrap environ_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "path_open",
            |mut caller: Caller<'_, _>,
             _dirfd: i32,
             _dirflags: i32,
             path: i32,
             path_len: i32,
             oflags: i32,
             _fs_rights_base: i64,
             _fs_rights_inheriting: i64,
             _fs_flags: i32,
             fd: i32| {
                debug_println!("path_open({_dirfd}, {_dirflags}, {path}, {path_len}, {oflags}, {_fs_rights_base}, {_fs_rights_inheriting}, {fd})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                let slice = &memory[path as usize..(path + path_len) as usize];
                let file_path = String::from_utf8_lossy(&slice).into();
                let result_value = fs::open(
                    file_path,
                    OpenFlags {
                        append: oflags & 0o2000 != 0,
                        exclude: oflags & 0o200 != 0,
                        truncate: oflags & 0o1000 != 0,
                    },
                ) as i32;
                memory[fd as usize..fd as usize + 4].copy_from_slice(&result_value.to_le_bytes());
                if result_value >= 0 { Ok(0) } else { Ok(44) }
            },
        )
        .expect("failed to wrap path_open");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "path_filestat_get",
            |mut caller: Caller<'_, _>,
             _fd: i32,
             _flags: i32,
             path: i32,
             path_len: i32,
             buf: i32| {
                debug_println!("path_filestat_get({_fd}, {_flags}, {path}, {path_len}, {buf})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                let slice = &memory[path as usize..(path + path_len) as usize];
                let name = String::from_utf8_lossy(&slice).into_owned();
                if name == "/" {
                    memory[buf as usize..buf as usize + 8].copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 8..buf as usize + 16]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 16..buf as usize + 24]
                        .copy_from_slice(&3u64.to_le_bytes());
                    memory[buf as usize + 24..buf as usize + 32]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 32..buf as usize + 40]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 40..buf as usize + 48]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 48..buf as usize + 56]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 56..buf as usize + 64]
                        .copy_from_slice(&0u64.to_le_bytes());
                    Ok(0)
                } else if fs::exists(&name) {
                    memory[buf as usize..buf as usize + 8].copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 8..buf as usize + 16]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 16..buf as usize + 24]
                        .copy_from_slice(&4u64.to_le_bytes());
                    memory[buf as usize + 24..buf as usize + 32]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 32..buf as usize + 40]
                        .copy_from_slice(&(fs::size(&name) as u64).to_le_bytes());
                    memory[buf as usize + 40..buf as usize + 48]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 48..buf as usize + 56]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 56..buf as usize + 64]
                        .copy_from_slice(&0u64.to_le_bytes());
                    Ok(0)
                } else {
                    Ok(44)
                }
            },
        )
        .expect("failed to wrap path_filestat_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_prestat_dir_name",
            |mut caller: Caller<'_, _>, fd: i32, name: i32, name_len: i32| {
                debug_println!("fd_prestat_dir_name({fd}, {name}, {name_len})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                if fd == 3 {
                    for i in 0..name_len.min(2) {
                        if i == 0 {
                            memory[name as usize] = b'/';
                        } else {
                            memory[name as usize + i as usize] = 0;
                        }
                    }
                }
                Ok(0)
            },
        )
        .expect("failed to wrap fd_prestat_dir_name");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_fdstat_get",
            |mut caller: Caller<'_, _>, fd: i32, result: i32| {
                debug_println!("fd_fdstat_get({fd}, {result})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                if fd == 3 {
                    memory[result as usize..result as usize + 8]
                        .copy_from_slice(&3u64.to_le_bytes());
                } else if fd < 3 || fd == 6 || fd == 7 || valid(fd as isize) {
                    memory[result as usize..result as usize + 8]
                        .copy_from_slice(&4u64.to_le_bytes());
                } else {
                    return Ok(8);
                }
                memory[result as usize + 8..result as usize + 16]
                    .copy_from_slice(&(-1i64).to_le_bytes());
                memory[result as usize + 16..result as usize + 24]
                    .copy_from_slice(&(-1i64).to_le_bytes());
                Ok(0)
            },
        )
        .expect("failed to wrap fd_fdstat_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_filestat_get",
            |mut caller: Caller<'_, _>, fd: i32, buf: i32| {
                debug_println!("fd_filestat_get({fd}, {buf})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                if fd == 3 {
                    memory[buf as usize..buf as usize + 8].copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 8..buf as usize + 16]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 16..buf as usize + 24]
                        .copy_from_slice(&3u64.to_le_bytes());
                    memory[buf as usize + 24..buf as usize + 32]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 32..buf as usize + 40]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 40..buf as usize + 48]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 48..buf as usize + 56]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 56..buf as usize + 64]
                        .copy_from_slice(&0u64.to_le_bytes());
                    Ok(0)
                } else if fs::valid(fd as isize) {
                    memory[buf as usize..buf as usize + 8].copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 8..buf as usize + 16]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 16..buf as usize + 24]
                        .copy_from_slice(&4u64.to_le_bytes());
                    memory[buf as usize + 24..buf as usize + 32]
                        .copy_from_slice(&1u64.to_le_bytes());
                    memory[buf as usize + 32..buf as usize + 40]
                        .copy_from_slice(&(fs::hsize(fd as isize) as u64).to_le_bytes());
                    memory[buf as usize + 40..buf as usize + 48]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 48..buf as usize + 56]
                        .copy_from_slice(&0u64.to_le_bytes());
                    memory[buf as usize + 56..buf as usize + 64]
                        .copy_from_slice(&0u64.to_le_bytes());
                    Ok(0)
                } else {
                    Ok(8)
                }
            },
        )
        .expect("failed to wrap fd_filestat_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_prestat_get",
            |mut caller: Caller<'_, _>, fd: i32, result: i32| {
                debug_println!("fd_prestat_get({fd}, {result})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                if fd == 3 {
                    memory[result as usize..result as usize + 4]
                        .copy_from_slice(&0u32.to_le_bytes());
                    memory[result as usize + 4..result as usize + 8]
                        .copy_from_slice(&2u32.to_le_bytes());
                } else if valid(fd as isize) {
                    memory[result as usize..result as usize + 4]
                        .copy_from_slice(&0u32.to_le_bytes());
                    memory[result as usize + 4..result as usize + 8]
                        .copy_from_slice(&(fs::path(fd as isize).len() as u32).to_le_bytes());
                } else {
                    return Ok(8);
                }
                Ok(0)
            },
        )
        .expect("failed to wrap fd_prestat_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_write",
            |mut caller: Caller<'_, _>, fd: i32, iovs: i32, iovs_len: i32, nwritten: i32| {
                debug_println!("fd_write({fd}, {iovs}, {iovs_len}, {nwritten})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                let mut count = 0i32;
                for iov_index in 0..iovs_len {
                    let iov = (iovs + iov_index * 8) as usize;
                    let base =
                        u32::from_le_bytes(memory[iov..iov + 4].try_into().unwrap()) as usize;
                    let size =
                        u32::from_le_bytes(memory[iov + 4..iov + 8].try_into().unwrap()) as usize;
                    let slice = &memory[base..base + size];
                    match fd {
                        0 => {}
                        1 | 2 => {
                            gprint!("{}", String::from_utf8_lossy(&slice));
                            count += size as i32;
                        }
                        handle => {
                            count += fs::write(handle as isize, &slice) as i32;
                        }
                    }
                }
                memory[nwritten as usize..nwritten as usize + 4]
                    .copy_from_slice(&count.to_le_bytes());
                Ok(0)
            },
        )
        .expect("failed to wrap fd_write");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_read",
            |mut caller: Caller<'_, _>, fd: i32, iovs: i32, iovs_len: i32, nread: i32| {
                debug_println!("fd_read({fd}, {iovs}, {iovs_len}, {nread})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                let mut count = 0i32;
                for iov_index in 0..iovs_len {
                    let iov = (iovs + iov_index * 8) as usize;
                    let base =
                        u32::from_le_bytes(memory[iov..iov + 4].try_into().unwrap()) as usize;
                    let size =
                        u32::from_le_bytes(memory[iov + 4..iov + 8].try_into().unwrap()) as usize;
                    let mut slice = &mut memory[base..base + size];
                    match fd {
                        0 => {}
                        1 | 2 => {}
                        handle => {
                            count += fs::read(handle as isize, &mut slice) as i32;
                        }
                    }
                }
                memory[nread as usize..nread as usize + 4].copy_from_slice(&count.to_le_bytes());
                Ok(0)
            },
        )
        .expect("failed to wrap fd_read");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_seek",
            |mut caller: Caller<'_, _>, fd: i32, offset: i64, whence: i32, result: i32| {
                debug_println!("fd_seek({fd}, {offset}, {whence}, {result})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                let result_value = fs::seek(fd as isize, offset as isize, whence);
                memory[result as usize..result as usize + 8]
                    .copy_from_slice(&result_value.to_le_bytes());
                if result_value >= 0 { Ok(0) } else { Ok(8) }
            },
        )
        .expect("failed to wrap fd_seek");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_close",
            |mut _caller: Caller<'_, _>, fd: i32| {
                debug_println!("fd_close({fd})");
                fs::close(fd as isize);
                Ok(0)
            },
        )
        .expect("failed to wrap fd_close");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "proc_exit",
            |mut _caller: Caller<'_, _>, code: i32| {
                println!("exited... status code {code}");
            },
        )
        .expect("failed to wrap proc_exit");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "clock_time_get",
            |mut caller: Caller<'_, _>, _clk_id: i32, _precision: i64, result: i32| {
                debug_println!("clock_time_get({_clk_id}, {_precision}, {result})");
                let Some(Extern::Memory(memory)) = caller.get_export("memory") else {
                    return Err(wasmi::Error::new("missing required WASI memory export"));
                };
                let (memory, _) = memory.data_and_store_mut(&mut caller);
                memory[result as usize..result as usize + 8].copy_from_slice(
                    &(unsafe { crate::irq::arch::TIMER * 10000000 }).to_le_bytes(),
                );
                Ok(0)
            },
        )
        .expect("failed to wrap clock_time_get");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "poll_oneoff",
            |mut _caller: Caller<'_, _>, _in: i32, _out: i32, _ns: i32, _ne: i32| {
                debug_println!("poll_oneoff({_in}, {_out}, {_ns}, {_ne})");
                Ok(0)
            },
        )
        .expect("failed to wrap poll_oneoff");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "path_create_directory",
            |mut _caller: Caller<'_, _>, _fd: i32, _path: i32, _path_len: i32| {
                debug_println!("path_create_directory({_fd}, {_path}, {_path_len})");
                Ok(0)
            },
        )
        .expect("failed to wrap path_create_directory");
    linker
        .func_wrap(
            "wasi_snapshot_preview1",
            "fd_fdstat_set_flags",
            |mut _caller: Caller<'_, _>, _fd: i32, _flags: i32| {
                debug_println!("fd_fdstat_set_flags({_fd}, {_flags})");
                Ok(0)
            },
        )
        .expect("failed to wrap fd_fdstat_set_flags");
}
