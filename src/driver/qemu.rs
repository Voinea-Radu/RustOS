use crate::utils::statics::ISA_DEBUG_EXIT_DEVICE_PORT;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
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

    loop{}
}