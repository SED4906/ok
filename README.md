# ok
OK is a kernel for [Wasm3](https://github.com/wasm3/wasm3).

Currently only supports x86_64, but could be ported.

Your CPU must support SSE. This is necessary for Wasm3.

This project uses the [Limine](https://github.com/limine-bootloader/limine) boot protocol.

## Getting started
```
$ cargo build --target x86_64-unknown-none
[put the generated kernel into a disk image with Limine]
$ qemu-system-x86_64 -bios path/to/OVMF.fd -hda path/to/DISK -m 512 -serial stdio
```
If you build with `--release` the debug messages from all the WASI support functions in `src/syscall.rs` will not show up.

## What works
- [x] The usual kernel things (memory management, interrupt handling, and a virtual filesystem)
- [x] Serial terminal output
- [x] WebAssembly interpreting with Wasm3
- [x] Some of WASI (namely the file I/O)
## What doesn't work
- [ ] Arguments and environment variables
- [ ] Pretty much anything else