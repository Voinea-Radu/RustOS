use crate::driver::interrupts::interrupts_handlers::InterruptIndex;
#[cfg(feature = "uefi")]
use crate::driver::interrupts::interrupts_handlers::IDT;
#[cfg(feature = "uefi")]
use acpi::AcpiTables;
use acpi::{AcpiHandler, PhysicalMapping};
use core::ptr::NonNull;
use lazy_static::lazy_static;
use spin::Mutex;
#[cfg(feature = "uefi")]
use x86_64::structures::paging::{FrameAllocator, Mapper, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};
use crate::println_serial;

lazy_static! {
    pub static ref  LAPIC_ADDR: Mutex<LAPICAddress> = Mutex::new(LAPICAddress::new()); // Needs to be initialized
}

#[derive(Debug, Clone, Copy)]
#[repr(isize)]
pub enum APICOffset {
    EOI = 0xb0,
    SVR = 0xF0,
    LVTTimer = 0x320,
    TDCR = 0x3E0,
    TimerInitialCount = 0x380,
    LVTKeyboard = 0x360,
}

pub struct LAPICAddress {
    address: *mut u32,
}

unsafe impl Send for LAPICAddress {}
unsafe impl Sync for LAPICAddress {}

impl LAPICAddress {
    pub fn new() -> Self {
        Self {
            address: core::ptr::null_mut()
        }
    }
}

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
    LAPIC_ADDR.lock().address = lapic_ptr;

    // Timer
    {
        let svr = lapic_ptr.offset(APICOffset::SVR as isize / 4);
        svr.write_volatile(svr.read_volatile() | 0x100); // Set bit 8

        let lvt_timer = lapic_ptr.offset(APICOffset::LVTTimer as isize / 4);
        lvt_timer.write_volatile(0x20 | (1 << 17)); // Vector 0x20, periodic mode

        let tdcr = lapic_ptr.offset(APICOffset::TDCR as isize / 4);
        tdcr.write_volatile(0x3);

        let timer_initial_count = lapic_ptr.offset(APICOffset::TimerInitialCount as isize / 4);
        timer_initial_count.write_volatile(0x100000);
    }

    // Keyboard
    {
        let lvt_keyboard = lapic_ptr.offset(APICOffset::LVTKeyboard as isize / 4);
        lvt_keyboard.write_volatile(InterruptIndex::Keyboard as u8 as u32);
    }
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
        Port::<u8>::new(0xA1).write(0xFF);
    }
}

#[cfg(feature = "uefi")]
pub fn apic_end_interrupt() {
    unsafe {
        let lapic_ptr = LAPIC_ADDR.lock().address;
        lapic_ptr.offset(APICOffset::EOI as isize / 4).write_volatile(0);
    }
}
