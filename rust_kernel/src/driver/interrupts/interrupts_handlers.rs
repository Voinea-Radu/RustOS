use lazy_static::lazy_static;
use crate::cpu::gdt::DOUBLE_FAULT_IST_INDEX;
use crate::{hlt_loop, print_serial, println_serial};
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
#[cfg(feature = "uefi")]
use crate::driver::interrupts::controller::apic::apic_end_interrupt;
#[cfg(feature = "bios")]
use crate::driver::interrupts::controller::pic::PICS;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(handle_breakpoint);
        idt.page_fault.set_handler_fn(handle_page_fault);

        idt[InterruptIndex::Timer as u8].set_handler_fn(handle_timer);
        //idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_interrupt_handler);

        unsafe {
            idt.double_fault
                .set_handler_fn(handle_double_fault)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

pub extern "x86-interrupt" fn handle_timer(_stack_frame: InterruptStackFrame) {
    print_serial!(".");

    #[cfg(feature = "bios")]
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }

    #[cfg(feature = "uefi")]
    apic_end_interrupt();
}

pub extern "x86-interrupt" fn handle_breakpoint(stack_frame: InterruptStackFrame) {
    println_serial!("Breakpoint hit:\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn handle_double_fault(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println_serial!("\nDouble fault:\n{:#?}", stack_frame);

    hlt_loop();
}

pub extern "x86-interrupt" fn handle_page_fault(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    println_serial!("Exception        : Page Fault");
    println_serial!("Accessed address : {:?}", Cr2::read());
    println_serial!("ErrorCode        : {:?}", error_code);
    println_serial!("{:#?}", stack_frame);

    hlt_loop();
}


