use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn irq_init() {
    unsafe {
        IDT.alignment_check.set_handler_fn(alignment_check);
        IDT.bound_range_exceeded.set_handler_fn(bound_range_exceeded);
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.cp_protection_exception.set_handler_fn(cp_protection_exception);
        IDT.debug.set_handler_fn(debug);
        IDT.device_not_available.set_handler_fn(device_not_available);
        IDT.divide_error.set_handler_fn(divide_error);
        IDT.double_fault.set_handler_fn(double_fault);
        IDT.general_protection_fault.set_handler_fn(general_protection_fault);
        IDT.hv_injection_exception.set_handler_fn(hv_injection_exception);
        IDT.invalid_opcode.set_handler_fn(invalid_opcode);
        IDT.invalid_tss.set_handler_fn(invalid_tss);
        IDT.machine_check.set_handler_fn(machine_check);
        IDT.non_maskable_interrupt.set_handler_fn(non_maskable_interrupt);
        IDT.overflow.set_handler_fn(overflow);
        IDT.page_fault.set_handler_fn(page_fault);
        IDT.security_exception.set_handler_fn(security_exception);
        IDT.segment_not_present.set_handler_fn(segment_not_present);
        IDT.simd_floating_point.set_handler_fn(simd_floating_point);
        IDT.stack_segment_fault.set_handler_fn(stack_segment_fault);
        IDT.virtualization.set_handler_fn(virtualization);
        IDT.vmm_communication_exception.set_handler_fn(vmm_communication_exception);
        IDT.x87_floating_point.set_handler_fn(x87_floating_point);
        IDT.load();
    }
}

extern "x86-interrupt" fn alignment_check(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn bound_range_exceeded(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn breakpoint(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn cp_protection_exception(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn debug(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn divide_error(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn device_not_available(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn double_fault(stack: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn general_protection_fault(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn hv_injection_exception(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn invalid_opcode(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn invalid_tss(stack: InterruptStackFrame,  error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn machine_check(stack: InterruptStackFrame) -> ! {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn non_maskable_interrupt(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn overflow(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn page_fault(stack: InterruptStackFrame, error_code: PageFaultErrorCode) {
    panic!("Unhandled interrupt ({error_code:?})");
}

extern "x86-interrupt" fn security_exception(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn segment_not_present(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn simd_floating_point(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn stack_segment_fault(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn virtualization(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}

extern "x86-interrupt" fn vmm_communication_exception(stack: InterruptStackFrame, error_code: u64) {
    panic!("Unhandled interrupt ({error_code:x})");
}

extern "x86-interrupt" fn x87_floating_point(stack: InterruptStackFrame) {
    panic!("Unhandled interrupt");
}
