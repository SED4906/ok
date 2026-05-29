# ok
OK is a kernel for Wasmi.

Currently only supports x86_64, but could be ported.

This project uses the [Limine](https://github.com/limine-bootloader/limine) boot protocol.

## Getting started
```
$ cargo build --target x86_64-unknown-none
[put the generated kernel into a disk image with Limine]
$ qemu-system-x86_64 -bios path/to/OVMF.fd -hda path/to/DISK -m 512 -serial stdio
```
If you build with `--release` the debug messages from all the WASI support functions in `src/syscall.rs` will not show up.

An example limine.conf might be:
```
/ok
protocol:limine
path:boot():/ok
cmdline:doom
module_path:boot():/wasidoom.wasm
module_path:boot():/doom1.wad
module_string:./doom1.wad
module_path:boot():/doom_profile
module_string:/etc/profile
```
doom_profile:
```
HOME=/
```
[doom1.wad and wasidoom.wasm](https://github.com/wasm3/pywasm3-doom-demo)

A module without a string specifies the WebAssembly code to run.

## What works
- [x] The usual kernel things (memory management, interrupt handling, and a virtual filesystem)
- [x] Serial debug output and graphical terminal output
- [x] WebAssembly interpreting with Wasmi
- [x] File I/O
- [x] Arguments and environment variables
## What doesn't work
- [ ] Input
