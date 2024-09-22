#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use lazy_static::lazy_static;
use rust_os::driver::qemu::{exit_qemu, QemuExitCode};
use rust_os::init;
use rust_os::test::tester::{
    all_tests_pass, pre_tests_run, run_test, test_fail, test_fail_with_error, test_pass,
};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

entry_point!(test_kernel_main);

//noinspection RsUnresolvedPath
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    init_test_idt();

    pre_tests_run(1);

    // trigger a stack overflow
    run_test(&stack_overflow);

    test_fail();
    exit_qemu(QemuExitCode::Fail)
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_fail_with_error(info);
}

#[allow(unconditional_recursion)]
fn stack_overflow() {
    pub fn infinite_recursion(count: i64) -> i64 {
        infinite_recursion(count + 1)
    }
    infinite_recursion(0);
}

lazy_static! {
    static ref TEST_IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        unsafe {
            idt.double_fault
                .set_handler_fn(test_double_handler)
                .set_stack_index(rust_os::kernel::global_descriptor_table::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

extern "x86-interrupt" fn test_double_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    test_pass();
    all_tests_pass();
}

fn init_test_idt() {
    TEST_IDT.load();
}
