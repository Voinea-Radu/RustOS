#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rust_os::utils::color::Color;
use rust_os::utils::color::ColorCode::LightCyan;
use rust_os::utils::statics::TROLL_MESSAGE;
use rust_os::{hlt_loop, print, println, println_color};

pub mod kernel {
    pub mod panic;
}

entry_point!(kernel_main);

//noinspection RsUnresolvedPath
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rust_os::init(boot_info);
    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main();
    hlt_loop();
}

pub fn main() {
    {
        println_color!("{}", TROLL_MESSAGE => Color::new_simple(LightCyan));
        println!("Hello {}", "Pudel Vesel!");

        print!("Hello ");
        println!("Pudel Prost!");
    }

    {
        let heap_value = Box::new(41);
        println!("heap_value at {:p}", heap_value);

        let mut vec = Vec::new();
        for i in 0..500 {
            vec.push(i);
        }
        println!("vec at {:p}", vec.as_slice());

        let reference_counted = Rc::new(vec![1, 2, 3]);
        let cloned_reference = reference_counted.clone();
        println!(
            "current reference count is {}",
            Rc::strong_count(&cloned_reference)
        );
        drop(reference_counted);
        println!(
            "reference count is {} now",
            Rc::strong_count(&cloned_reference)
        );
    }

    // panic!("Pudelul si Daria au iesit la cafea!");
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!(
        "Panicked at {}: {}",
        info.location().unwrap(),
        info.message()
    );
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test::tester::test_fail_with_error(info);
}
