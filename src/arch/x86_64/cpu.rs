use x86::{
    controlregs::{Cr0, Cr4, cr0, cr0_write, cr4, cr4_write},
    io::{inb, outb},
};

const PIC1: u16 = 0x20;
const PIC2: u16 = 0xA0;

pub fn cpu_init() {
    unsafe {
        outb(PIC1, 0x11);
        outb(PIC2, 0x11);
        outb(PIC1 + 1, 0x20);
        outb(PIC2 + 1, 0x28);
        outb(PIC1 + 1, 4);
        outb(PIC2 + 1, 2);
        outb(PIC1 + 1, 0x01);
        outb(PIC2 + 1, 0x01);
        outb(PIC1 + 1, 0xFF);
        outb(PIC2 + 1, 0xFF);
        outb(0x43, 0x34);
        outb(0x40, 0x9C);
        outb(0x40, 0x2E);
        let mut mask = inb(PIC1 + 1);
        mask &= !1;
        outb(PIC1 + 1, mask);
        x86::irq::enable();
    }
}
