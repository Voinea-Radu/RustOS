use crate::binaries::shell::Shell;
use crate::kernel::interrupts::{InterruptIndex, PICS};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;
use x86_64::structures::idt::InterruptStackFrame;
use crate::println;

pub const KEYBOARD_PORT: u16 = 0x60;


lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::Ignore
    ));
    static ref SHELL: Shell = Shell::new();
    static ref KEYBOARD_MANAGER: KeyboardManager<'static> = KeyboardManager::new(&*SHELL);
}

pub trait KeyboardListener: Sync {
    fn on_key(&self, key: KeyCode);

    fn on_char(&self, char: char);
}

pub struct KeyboardManager<'a> {
    active_listener: &'a dyn KeyboardListener,
}

impl<'a> KeyboardManager<'a> {
    pub fn new(listener: &'a dyn KeyboardListener) -> Self {
        KeyboardManager {
            active_listener: listener,
        }
    }

    pub fn handle_key(&self, key: KeyCode) {
        self.active_listener.on_key(key);
    }

    pub fn handle_char(&self, c: char) {
        self.active_listener.on_char(c);
    }
}


pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::RawKey(key) => {
                    KEYBOARD_MANAGER.handle_key(key);
                }
                DecodedKey::Unicode(char) => {
                    KEYBOARD_MANAGER.handle_char(char);
                }
            }
        }
    }

    // print!("{}", scancode);

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard as u8)
    }
}