use crate::binaries::shell::Shell;
use crate::kernel::interrupts::{InterruptIndex, PICS};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
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
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}

pub trait KeyboardListener: Send {
    fn handle_key(&mut self, key: KeyCode);

    fn handle_char(&mut self, char: char);
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::RawKey(key) => {
                    SHELL.lock().handle_key(key);
                }
                DecodedKey::Unicode(char) => {
                    SHELL.lock().handle_char(char);
                }
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard as u8)
    }
}
