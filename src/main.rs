#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]

use bootloader::{entry_point, BootInfo};
use core::ops::Add;
use core::panic::PanicInfo;
use rust_os::kernel::memory::{create_example_mapping, BootInfoFrameAllocator, EmptyFrameAllocator};
use rust_os::utils::color::Color;
use rust_os::utils::color::ColorCode::LightCyan;
use rust_os::utils::statics::TROLL_MESSAGE;
use rust_os::{hlt_loop, print, println, println_color};
use x86_64::structures::paging::{Page, Translate};
use x86_64::VirtAddr;

pub mod kernel {
    pub mod panic;
}

entry_point!(kernel_main);

//noinspection RsUnresolvedPath
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    rust_os::init();

    #[cfg(test)]
    test_main();

    #[cfg(not(test))]
    main();

    let physical_memory_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = rust_os::kernel::memory::init(physical_memory_offset);
    let mut frame_allocator = BootInfoFrameAllocator::new(&boot_info.memory_map);

    let page = Page::containing_address(VirtAddr::new(0x0));
    create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_pointer: *mut u64 = page.start_address().as_mut_ptr();
    unsafe {
        page_pointer
            .offset(400)
            .write_volatile(0x_f021_f077_f065_f04e)
    }

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses{
        let virtual_address = VirtAddr::new(address);
        let physical_address = mapper.translate_addr(virtual_address);

        println!("{:?} -> {:?}", virtual_address, physical_address);
    }

    hlt_loop()
}

pub fn main() {
    println_color!("{}", TROLL_MESSAGE => Color::new_simple(LightCyan));
    println!("Hello {}", "Pudel Vesel!");

    print!("Hello ");
    println!("Pudel Prost!");

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
