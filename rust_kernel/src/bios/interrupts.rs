use crate::global_descriptor_table::DOUBLE_FAULT_IST_INDEX;
use crate::{hlt_loop, print_serial, println_serial};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Timer as u8].set_handler_fn(timer_handler);
        //idt[InterruptIndex::Keyboard as u8].set_handler_fn(keyboard_interrupt_handler);

        unsafe {
           idt.double_fault
               .set_handler_fn(double_fault_handler)
               .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }

        idt
    };
}

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println_serial!("Breakpoint hit:\n{:#?}", stack_frame);
}


extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    println_serial!("\nDouble fault:\n{:#?}", stack_frame);

    hlt_loop();
}

extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    print_serial!(".");
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer as u8);
    }
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: PageFaultErrorCode) {
    println_serial!("Exception        : Page Fault");
    println_serial!("Accessed address : {:?}", Cr2::read());
    println_serial!("ErrorCode        : {:?}", error_code);
    println_serial!("{:#?}", stack_frame);

    hlt_loop();
}