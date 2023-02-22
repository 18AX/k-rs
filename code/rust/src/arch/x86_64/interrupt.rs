use core::arch::asm;

use x86_64::structures::idt::{
    HandlerFunc, InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode,
};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init() {
    unsafe {
        IDT.divide_error.set_handler_fn(division_error);
        IDT.debug.set_handler_fn(debug);
        IDT.non_maskable_interrupt.set_handler_fn(nmi);
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.overflow.set_handler_fn(overflow);
        IDT.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded);
        IDT.invalid_opcode.set_handler_fn(invalid_opcode);
        IDT.device_not_available
            .set_handler_fn(device_not_available);
        IDT.double_fault.set_handler_fn(double_fault);
        IDT.invalid_tss.set_handler_fn(invalid_tss);
        IDT.segment_not_present.set_handler_fn(segment_not_present);
        IDT.stack_segment_fault.set_handler_fn(stack_segment_fault);
        IDT.general_protection_fault
            .set_handler_fn(general_protection_fault);
        IDT.page_fault.set_handler_fn(page_fault);
        IDT.x87_floating_point.set_handler_fn(x87_floating_point);
        IDT.alignment_check.set_handler_fn(alignment_check);
        IDT.machine_check.set_handler_fn(machine_check);
        IDT.simd_floating_point.set_handler_fn(simd_floating_point);
        IDT.virtualization.set_handler_fn(virtualization);
        IDT.vmm_communication_exception
            .set_handler_fn(vmm_communication_exception);
        IDT.security_exception.set_handler_fn(security_exception);

        IDT.load();
    };
}

pub fn enable_irq(state: bool) {
    match state {
        true => unsafe {
            asm!("sti");
        },
        false => unsafe {
            asm!("cli");
        },
    }
}

pub fn register_handler(index: u8, handler: HandlerFunc) {
    unsafe {
        IDT[index as usize].set_handler_fn(handler);
    };
}

extern "x86-interrupt" fn division_error(stack_frame: InterruptStackFrame) {
    panic!("Division error: {:?}", stack_frame);
}

extern "x86-interrupt" fn debug(stack_frame: InterruptStackFrame) {
    panic!("Debug: {:?}", stack_frame);
}

extern "x86-interrupt" fn nmi(stack_frame: InterruptStackFrame) {
    panic!("Non maskable interrupt: {:?}", stack_frame);
}

extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame) {
    panic!("Breakpoint: {:?}", stack_frame);
}

extern "x86-interrupt" fn overflow(stack_frame: InterruptStackFrame) {
    panic!("Overflow: {:?}", stack_frame);
}

extern "x86-interrupt" fn bound_range_exceeded(stack_frame: InterruptStackFrame) {
    panic!("Bound range exceeded: {:?}", stack_frame);
}

extern "x86-interrupt" fn invalid_opcode(stack_frame: InterruptStackFrame) {
    panic!("Invalid opcode: {:?}", stack_frame);
}

extern "x86-interrupt" fn device_not_available(stack_frame: InterruptStackFrame) {
    panic!("Device not available: {:?}", stack_frame);
}

extern "x86-interrupt" fn double_fault(stack_frame: InterruptStackFrame, error_code: u64) -> ! {
    panic!("Double fault({:x}): {:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn invalid_tss(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("Invalid_tss({:x}): {:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn segment_not_present(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("Segment not present({:x}): {:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn stack_segment_fault(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("Stack segment fault({:x}): {:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn general_protection_fault(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "General protection fault({:x}): {:?}",
        error_code, stack_frame
    );
}

extern "x86-interrupt" fn page_fault(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    panic!("Page fault({:?}): {:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn x87_floating_point(stack_frame: InterruptStackFrame) {
    panic!("x87 floating point: {:?}", stack_frame);
}

extern "x86-interrupt" fn alignment_check(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("Alignment check({:x}): {:?}", error_code, stack_frame);
}

extern "x86-interrupt" fn machine_check(stack_frame: InterruptStackFrame) -> ! {
    panic!("Machine check: {:?}", stack_frame);
}

extern "x86-interrupt" fn simd_floating_point(stack_frame: InterruptStackFrame) {
    panic!("Simd floating point: {:?}", stack_frame);
}

extern "x86-interrupt" fn virtualization(stack_frame: InterruptStackFrame) {
    panic!("virtualization: {:?}", stack_frame);
}

extern "x86-interrupt" fn vmm_communication_exception(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    panic!(
        "VMM communication exception({:x}): {:?}",
        error_code, stack_frame
    );
}

extern "x86-interrupt" fn security_exception(stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("Security exception({:x}): {:?}", error_code, stack_frame);
}
