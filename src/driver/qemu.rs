use crate::hlt_loop;

pub const ISA_DEBUG_EXIT_DEVICE_PORT: u16 = 0xf4; // Usually unused device port

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
#[allow(dead_code)]
pub enum QemuExitCode {
    Success = 0x10,
    Fail = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(ISA_DEBUG_EXIT_DEVICE_PORT);
        port.write(exit_code as u32)
    }

    hlt_loop()
}
