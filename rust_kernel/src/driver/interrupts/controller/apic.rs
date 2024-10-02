use acpi::{AcpiHandler, PhysicalMapping};
use core::ptr::NonNull;
use x86_64::{PhysAddr, VirtAddr};
#[cfg(feature = "uefi")]
use crate::driver::interrupts::interrupts_handlers::IDT;
#[cfg(feature = "uefi")]
use x86_64::structures::paging::{FrameAllocator, Mapper, PhysFrame, Size4KiB};
#[cfg(feature = "uefi")]
use acpi::AcpiTables;


pub const APIC_EOI_OFFSET: isize = 0xB0;
pub static mut LAPIC_ADDR: *mut u32 = core::ptr::null_mut(); // Needs to be initialized

pub struct AcpiHandlerImpl {
    physical_memory_offset: VirtAddr,
}

impl AcpiHandlerImpl {
    pub fn new(physical_memory_offset: VirtAddr) -> Self {
        Self { physical_memory_offset }
    }
}

unsafe impl Send for AcpiHandlerImpl {}
unsafe impl Sync for AcpiHandlerImpl {}

impl Clone for AcpiHandlerImpl {
    fn clone(&self) -> Self {
        Self {
            physical_memory_offset: self.physical_memory_offset,
        }
    }
}

impl AcpiHandler for AcpiHandlerImpl {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> PhysicalMapping<Self, T> {
        let phys_addr = PhysAddr::new(physical_address as u64);
        let virt_addr = self.physical_memory_offset + phys_addr.as_u64();

        PhysicalMapping::new(
            physical_address,
            NonNull::new(virt_addr.as_mut_ptr()).expect("Failed to get virtual address"),
            size,
            size,
            self.clone(),
        )
    }

    fn unmap_physical_region<T>(_region: &PhysicalMapping<Self, T>) {
        // No unmapping necessary as we didn't create any new mappings
    }
}

#[cfg(feature = "uefi")]
pub unsafe fn init(
    rsdp: usize, physical_memory_offset: VirtAddr,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let handler = AcpiHandlerImpl::new(physical_memory_offset);
    let acpi_tables = AcpiTables::from_rsdp(handler, rsdp).expect("Failed to parse ACPI tables");
    let platform_info = acpi_tables.platform_info().expect("Failed to get platform info");
    match platform_info.interrupt_model {
        acpi::InterruptModel::Apic(apic) => {
            let local_apic_address = apic.local_apic_address;
            init_local_apic(local_apic_address as usize, mapper, frame_allocator);
        }
        _ => {
            // Handle other interrupt models if necessary
        }
    }

    disable_pic();
    x86_64::instructions::interrupts::enable();
    IDT.load();
}

#[cfg(feature = "uefi")]
unsafe fn init_local_apic(
    local_apic_addr: usize,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    let virt_addr = map_apic(
        local_apic_addr as u64,
        mapper,
        frame_allocator,
    );

    let lapic_ptr = virt_addr.as_mut_ptr::<u32>();

    // Store the LAPIC address for later use in the interrupt handler
    LAPIC_ADDR = lapic_ptr; // If RustRover reports an error chances are that it's wrong. It should compile just fine.

    const APIC_SVR_OFFSET: isize = 0xF0;
    const APIC_LVT_TIMER_OFFSET: isize = 0x320;
    const APIC_TDCR_OFFSET: isize = 0x3E0;
    const APIC_TIMER_INITIAL_COUNT_OFFSET: isize = 0x380;

    let svr = lapic_ptr.offset(APIC_SVR_OFFSET / 4);
    svr.write_volatile(svr.read_volatile() | 0x100); // Set bit 8

    let lvt_timer = lapic_ptr.offset(APIC_LVT_TIMER_OFFSET / 4);
    lvt_timer.write_volatile(0x20 | (1 << 17)); // Vector 0x20, periodic mode

    let tdcr = lapic_ptr.offset(APIC_TDCR_OFFSET / 4);
    tdcr.write_volatile(0x3);

    let timer_initial_count = lapic_ptr.offset(APIC_TIMER_INITIAL_COUNT_OFFSET / 4);
    timer_initial_count.write_volatile(0x100000);
}

#[cfg(feature = "uefi")]
fn map_apic(
    physical_address: u64,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> VirtAddr {
    use x86_64::structures::paging::Page;
    use x86_64::structures::paging::PageTableFlags as Flags;

    let phys_addr = PhysAddr::new(physical_address);
    let page = Page::containing_address(VirtAddr::new(phys_addr.as_u64()));
    let frame = PhysFrame::containing_address(phys_addr);

    let flags = Flags::PRESENT | Flags::WRITABLE | Flags::NO_CACHE;

    unsafe {
        mapper
            .map_to(page, frame, flags, frame_allocator)
            .expect("APIC mapping failed")
            .flush();
    }

    page.start_address()
}

#[cfg(feature = "uefi")]
fn disable_pic() {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut pic1 = Port::<u8>::new(0xA1);
        let mut pic2 = Port::<u8>::new(0x21);

        pic1.write(0xFF);
        pic2.write(0xFF);
    }
}

#[cfg(feature = "uefi")]
pub fn apic_end_interrupt() {
    unsafe {
        const APIC_EOI_OFFSET: isize = 0xB0;
        let lapic_ptr = LAPIC_ADDR;
        lapic_ptr.offset(APIC_EOI_OFFSET / 4).write_volatile(0);
    }
}
