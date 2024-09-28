use crate::memory::frame_allocator::BootInfoFrameAllocator;
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use core::ptr::NonNull;
use x86_64::structures::paging::{FrameAllocator, Mapper, OffsetPageTable, Page, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};

struct AcpiHandlerImpl {
    mapper: &'static mut OffsetPageTable<'static>,
    frame_allocator: &'static mut BootInfoFrameAllocator,
}

impl AcpiHandlerImpl {
    pub fn new(mapper: &'static mut OffsetPageTable<'static>, frame_allocator: &'static mut BootInfoFrameAllocator) -> Self {
        Self {
            mapper,
            frame_allocator,
        }
    }
}

unsafe impl Send for AcpiHandlerImpl {}
unsafe impl Sync for AcpiHandlerImpl {}

impl Clone for AcpiHandlerImpl {
    fn clone(&self) -> Self {
        Self::new(self.mapper, self.frame_allocator)
    }
}

impl AcpiHandler for AcpiHandlerImpl {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        use x86_64::{
            structures::paging::{Mapper, Page, PageTableFlags, PhysFrame, Size4KiB}, PhysAddr,
            VirtAddr,
        };

        let phys_addr = PhysAddr::new(physical_address as u64);
        let virt_addr = VirtAddr::new(physical_address as u64); // Identity mapping

        let page = Page::<Size4KiB>::containing_address(virt_addr);
        let frame = PhysFrame::<Size4KiB>::containing_address(phys_addr);

        let flags = PageTableFlags::PRESENT | PageTableFlags::READ_WRITE;

        self.mapper
            .map_to(page, frame, flags, self.frame_allocator)
            .expect("Failed to map frame")
            .flush();

        PhysicalMapping::new(
            physical_address,
            NonNull::new(virt_addr.as_mut_ptr()).expect("Failed to get virtual address"),
            size,
            size,
            self.clone(),
        )
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // Implement unmapping logic if necessary
    }
}

pub unsafe fn init(rsdp: usize, mapper: &'static mut OffsetPageTable<'static>, frame_allocator: &'static mut BootInfoFrameAllocator) {
    let acpi_tables = AcpiTables::from_rsdp(AcpiHandlerImpl::new(mapper, frame_allocator), rsdp).expect("TODO ERROR");
    let platform_info = acpi_tables.platform_info().expect("TODO ERROR");
    match platform_info.interrupt_model {
        acpi::InterruptModel::Apic(apic) => {
            // You can access local_apic and io_apics here
            let local_apic_address = apic.local_apic_address;
            let io_apics = apic.io_apics;

            init_local_apic(local_apic_address as usize, mapper, frame_allocator);

            // Initialize Local APIC
            // Initialize IO APIC(s)
        }
        _ => {
            // Handle other interrupt models if necessary
        }
    }

    disable_pic();
    unsafe {
        x86_64::instructions::interrupts::enable();
    }

}

fn disable_pic() {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut pic1 = Port::<u8>::new(0xA1);
        let mut pic2 = Port::<u8>::new(0x21);

        pic1.write(0xFF);
        pic2.write(0xFF);
    }
}

/// Initializes the Local APIC by mapping its physical address into the virtual address space
/// and enabling it via the Spurious Interrupt Vector Register (SVR).
///
/// # Safety
///
/// This function is unsafe because it performs raw pointer dereferencing and writes to hardware
/// registers. Ensure that the `local_apic_addr` is correct and that paging is correctly set up.
unsafe fn init_local_apic(
    local_apic_addr: usize,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    // Map the Local APIC physical address to a virtual address
    let phys_addr = PhysAddr::new(local_apic_addr as u64);
    let virt_addr = VirtAddr::new(phys_addr.as_u64()); // Identity mapping; adjust if needed

    // Calculate the page and frame containing the Local APIC
    let page = Page::<Size4KiB>::containing_address(virt_addr);
    let frame = PhysFrame::<Size4KiB>::containing_address(phys_addr);

    // Set the page table flags
    let flags = PageTableFlags::PRESENT
        | PageTableFlags::WRITABLE
        | PageTableFlags::NO_CACHE
        | PageTableFlags::WRITE_THROUGH;

    // Map the page to the physical frame
    mapper
        .map_to(page, frame, flags, frame_allocator)
        .expect("Failed to map Local APIC")
        .flush();

    // The Local APIC registers are memory-mapped and can be accessed via the virtual address
    let lapic_ptr = virt_addr.as_mut_ptr::<u32>();

    // Define offsets for the Local APIC registers (in bytes)
    const APIC_SVR_OFFSET: isize = 0xF0;
    const APIC_LVT_TIMER_OFFSET: isize = 0x320;
    const APIC_TDCR_OFFSET: isize = 0x3E0;
    const APIC_TIMER_INITIAL_COUNT_OFFSET: isize = 0x380;

    // Enable the Local APIC by setting the Spurious Interrupt Vector Register (SVR)
    let svr = lapic_ptr.offset(APIC_SVR_OFFSET / 4);
    svr.write_volatile(svr.read_volatile() | 0x100); // Set bit 8 to enable the APIC

    // Additional initialization steps can be performed here
    // For example, setting up the Local Vector Table (LVT) entries, timer, etc.

    // Example: Initialize the Local APIC timer
    // Disable the timer by masking it
    let lvt_timer = lapic_ptr.offset(APIC_LVT_TIMER_OFFSET / 4);
    lvt_timer.write_volatile(0x10000); // Mask the timer

    // Set the Timer Divide Configuration Register (TDCR) to divide by 16
    let tdcr = lapic_ptr.offset(APIC_TDCR_OFFSET / 4);
    tdcr.write_volatile(0x3);

    // Set the Initial Count register to zero
    let timer_initial_count = lapic_ptr.offset(APIC_TIMER_INITIAL_COUNT_OFFSET / 4);
    timer_initial_count.write_volatile(0);
}


