use crate::driver::display::cursor::CURSOR;
use crate::driver::interrupts::interrupts_handlers::end_interrupt;
use crate::print_serial;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;

pub const KEYBOARD_PORT: u16 = 0x60;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::Us104Key,
            HandleControl::Ignore
        ));
}

pub extern "x86-interrupt" fn handle_keyboard(_stack_frame: InterruptStackFrame) {
    use core::fmt::Write;

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::RawKey(key) => {
                    write!(CURSOR.lock(), "{:?}", key).unwrap();
                    print_serial!( "{:?}", key);
                }
                DecodedKey::Unicode(char) => {
                    write!(CURSOR.lock(), "{}", char).unwrap();
                    print_serial!( "{}", char);
                }
            }
        }
    }

    end_interrupt();
}