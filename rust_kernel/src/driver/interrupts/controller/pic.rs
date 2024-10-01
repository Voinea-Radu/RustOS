use crate::cpu::gdt;
use crate::driver::interrupts::interrupts_handlers;
use crate::driver::interrupts::interrupts_handlers::{PIC_1_OFFSET, PIC_2_OFFSET};
use pic8259::ChainedPics;
use spin::Mutex;

pub static PICS: Mutex<ChainedPics> =
    Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init() {
    gdt::init();
    interrupts_handlers::IDT.load();
    unsafe {
        PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}