#![no_std]
#![no_main]
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test::tester::test_runner)]

use core::ops::Add;
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::structures::paging::{Page, PageTable};
use x86_64::VirtAddr;
use rust_os::utils::color::Color;
use rust_os::utils::color::ColorCode::LightCyan;
use rust_os::utils::statics::TROLL_MESSAGE;
use rust_os::{hlt_loop, print, println, println_color};
use rust_os::kernel::memory::{active_level4_table, translate_address};

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
    let level4_table = active_level4_table(physical_memory_offset);

    for (index, entry) in level4_table.iter().enumerate(){
        if !entry.is_unused(){
            println!("L4 Entry: {}: {:?}", index, entry);

            let physical_address = entry.frame().unwrap().start_address();
            let virtual_address = physical_address.as_u64() + boot_info.physical_memory_offset;
            let virtual_address_pointer = VirtAddr::new(virtual_address).as_mut_ptr();
            let level3_table: &PageTable = unsafe{
                &*virtual_address_pointer
            };

            for (index, entry) in level3_table.iter().enumerate(){
                if !entry.is_unused(){
                    println!("  L3 Entry: {}: {:?}", index, entry);
                }
            }
        }
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
        let physical_address = translate_address(virtual_address, physical_memory_offset);

        println!("{:?} -> {:?}", virtual_address, physical_address);
    }

    hlt_loop()
}

pub fn main() {
    println_color!("{}", TROLL_MESSAGE => Color::new_simple(LightCyan));
    println!("Hello {}", "Pudel Vesel!\n");

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
