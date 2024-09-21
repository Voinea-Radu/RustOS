use x86_64::registers::control::Cr3;
use x86_64::structures::paging::page_table::FrameError;
use x86_64::structures::paging::PageTable;
use x86_64::{PhysAddr, VirtAddr};

pub fn active_level4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    let (level4_table_frame, _) = Cr3::read();

    let physical_address = level4_table_frame.start_address();
    let virtual_address = physical_memory_offset + physical_address.as_u64();
    let page_table_pointer: *mut PageTable = virtual_address.as_mut_ptr();

    unsafe {
        &mut *page_table_pointer
    }
}

pub fn translate_address(virtual_address: VirtAddr, physical_memory_offset: VirtAddr) -> Option<PhysAddr> {
    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        virtual_address.p4_index(), virtual_address.p3_index(), virtual_address.p2_index(), virtual_address.p1_index()
    ];

    let mut frame = level_4_table_frame;

    for &index in &table_indexes {
        let virtual_address = physical_memory_offset + frame.start_address().as_u64();
        let table_pointer: *const PageTable = virtual_address.as_mut_ptr();
        let table = unsafe {
            &*table_pointer
        };

        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("HugeFrame not support"),
        }
    }

    Some(frame.start_address() + u64::from(virtual_address.page_offset()))
}

