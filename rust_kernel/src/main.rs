#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader_api::{entry_point, BootInfo};
use log::info;
use core::fmt::Write;
use crate::QemuExitCode::Success;

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

entry_point!(kernel_main, config = &CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    writeln!(serial(), "hello1").unwrap();
    writeln!(serial(), "hello2").unwrap();

    //exit_qemu(Success);
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> !{
    loop{}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::{nop, port::Port};
    writeln!(serial(), "hello3").unwrap();

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    writeln!(serial(), "hello4").unwrap();

    loop {
        nop();
    }
}

pub fn serial() -> uart_16550::SerialPort {
    let mut port = unsafe { uart_16550::SerialPort::new(0x3F8) };
    port.init();
    port
}
