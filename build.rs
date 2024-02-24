fn main() {
    cc::Build::new()
    .file("src/vsnprintf.c")
    .compile("vsnprintf");
    // Tell cargo to pass the linker script to the linker..
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    // ..and to re-run if it changes.
    println!("cargo:rerun-if-changed=linker.ld");
}
