use spin::Mutex;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static IDT: Mutex<InterruptDescriptorTable> = Mutex::new(InterruptDescriptorTable::new());

pub fn irq_init() {
    let mut idt: spin::MutexGuard<'_, InterruptDescriptorTable> = IDT.lock();
    unsafe {
        idt.alignment_check.set_handler_fn(alignment_check);
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded);
        idt.breakpoint.set_handler_fn(breakpoint);
        idt.cp_protection_exception
            .set_handler_fn(cp_protection_exception);
        idt.debug.set_handler_fn(debug);
        idt.device_not_available
            .set_handler_fn(device_not_available);
        idt.divide_error.set_handler_fn(divide_error);
        idt.double_fault.set_handler_fn(double_fault);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault);
        idt.hv_injection_exception
            .set_handler_fn(hv_injection_exception);
        idt.invalid_opcode.set_handler_fn(invalid_opcode);
        idt.invalid_tss.set_handler_fn(invalid_tss);
        idt.machine_check.set_handler_fn(machine_check);
        idt.non_maskable_interrupt
            .set_handler_fn(non_maskable_interrupt);
        idt.overflow.set_handler_fn(overflow);
        idt.page_fault.set_handler_fn(page_fault);
        idt.security_exception.set_handler_fn(security_exception);
        idt.segment_not_present.set_handler_fn(segment_not_present);
        idt.simd_floating_point.set_handler_fn(simd_floating_point);
        idt.stack_segment_fault.set_handler_fn(stack_segment_fault);
        idt.virtualization.set_handler_fn(virtualization);
        idt.vmm_communication_exception
            .set_handler_fn(vmm_communication_exception);
        idt.x87_floating_point.set_handler_fn(x87_floating_point);
        idt.load_unsafe();
    }
}

extern "x86-interrupt" fn alignment_check(_stack: InterruptStackFrame, error_code: u64) {
    panic!("Alignment Check ({error_code:x})");
}

extern "x86-interrupt" fn bound_range_exceeded(_stack: InterruptStackFrame) {
    panic!("Bound Range Exceeded");
}

extern "x86-interrupt" fn breakpoint(_stack: InterruptStackFrame) {
    panic!("Breakpoint");
}

extern "x86-interrupt" fn cp_protection_exception(_stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn debug(_stack: InterruptStackFrame) {
    panic!("Debug");
}

extern "x86-interrupt" fn divide_error(_stack: InterruptStackFrame) {
    panic!("Divide Error");
}

extern "x86-interrupt" fn device_not_available(_stack: InterruptStackFrame) {
    panic!("Device Not Available");
}

extern "x86-interrupt" fn double_fault(_stack: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Double Fault ({error_code:x})");
}

extern "x86-interrupt" fn general_protection_fault(_stack: InterruptStackFrame, error_code: u64) {
    panic!("General Protection Fault ({error_code:x})");
}

extern "x86-interrupt" fn hv_injection_exception(_stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn invalid_opcode(_stack: InterruptStackFrame) {
    panic!("Invalid Opcode");
}

extern "x86-interrupt" fn invalid_tss(_stack: InterruptStackFrame, error_code: u64) {
    panic!("Invalid TSS ({error_code:x})");
}

extern "x86-interrupt" fn machine_check(_stack: InterruptStackFrame) -> ! {
    panic!("Machine Check");
}

extern "x86-interrupt" fn non_maskable_interrupt(_stack: InterruptStackFrame) {
    panic!("Non Maskable Interrupt");
}

extern "x86-interrupt" fn overflow(_stack: InterruptStackFrame) {
    panic!("Overflow");
}

extern "x86-interrupt" fn page_fault(_stack: InterruptStackFrame, error_code: PageFaultErrorCode) {
    panic!("Page Fault ({error_code:?})");
}

extern "x86-interrupt" fn security_exception(_stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn segment_not_present(_stack: InterruptStackFrame, error_code: u64) {
    panic!("Segment Not Present ({error_code:x})");
}

extern "x86-interrupt" fn simd_floating_point(_stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn stack_segment_fault(_stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn virtualization(_stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn vmm_communication_exception(
    _stack: InterruptStackFrame,
    error_code: u64,
) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn x87_floating_point(_stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}
