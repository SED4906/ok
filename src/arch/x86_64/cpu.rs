use x86::controlregs::{Cr0,Cr4,cr0,cr0_write,cr4,cr4_write};

pub fn cpu_init() {
    unsafe {
        cr0_write(cr0().difference(Cr0::from_bits(1<<2).unwrap()));
        cr0_write(cr0().union(Cr0::from_bits(1<<1).unwrap()));
        cr4_write(cr4().union(Cr4::from_bits(3<<9).unwrap()));
    }
}