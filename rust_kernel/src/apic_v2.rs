use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use core::ptr::NonNull;
use x86_64::structures::paging::{FrameAllocator, Mapper, Page, PageTableFlags, PhysFrame, Size4KiB};
use x86_64::{PhysAddr, VirtAddr};
use crate::println_serial;

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

pub unsafe fn init(rsdp: usize, physical_memory_offset: VirtAddr) {
    println_serial!("1");
    let handler = AcpiHandlerImpl::new(physical_memory_offset);
    println_serial!("2");
    let acpi_tables = AcpiTables::from_rsdp(handler, rsdp).expect("Failed to parse ACPI tables");
    println_serial!("3");
    let platform_info = acpi_tables.platform_info().expect("Failed to get platform info");
    println_serial!("4");
    match platform_info.interrupt_model {
        acpi::InterruptModel::Apic(apic) => {
            println_serial!("5");
            let local_apic_address = apic.local_apic_address;
            println_serial!("6");
            init_local_apic(local_apic_address as usize, physical_memory_offset);
            println_serial!("7");
        }
        _ => {
            println_serial!("8");
            // Handle other interrupt models if necessary
        }
    }

    println_serial!("9");
    disable_pic();
    println_serial!("10");
    x86_64::instructions::interrupts::enable();
    println_serial!("11");
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

unsafe fn init_local_apic(
    local_apic_addr: usize,
    physical_memory_offset: VirtAddr,
) {
    println_serial!("20");
    let phys_addr = PhysAddr::new(local_apic_addr as u64);
    println_serial!("21");
    let virt_addr = physical_memory_offset + phys_addr.as_u64();
    println_serial!("22");

    println_serial!("23");
    let lapic_ptr = virt_addr.as_mut_ptr::<u32>();
    println_serial!("24");

    println_serial!("25");
    const APIC_SVR_OFFSET: isize = 0xF0;
    println_serial!("26");
    const APIC_LVT_TIMER_OFFSET: isize = 0x320;
    println_serial!("27");
    const APIC_TDCR_OFFSET: isize = 0x3E0;
    println_serial!("28");
    const APIC_TIMER_INITIAL_COUNT_OFFSET: isize = 0x380;
    println_serial!("29");

    println_serial!("30");
    let svr = lapic_ptr.offset(APIC_SVR_OFFSET / 4);
    println_serial!("31");
    svr.write_volatile(svr.read_volatile() | 0x100); // Set bit 8 to enable the APIC
    println_serial!("32");

    println_serial!("33");
    let lvt_timer = lapic_ptr.offset(APIC_LVT_TIMER_OFFSET / 4);
    println_serial!("34");
    lvt_timer.write_volatile(0x10000); // Mask the timer
    println_serial!("35");

    println_serial!("36");
    let tdcr = lapic_ptr.offset(APIC_TDCR_OFFSET / 4);
    println_serial!("37");
    tdcr.write_volatile(0x3);
    println_serial!("38");

    println_serial!("39");
    let timer_initial_count = lapic_ptr.offset(APIC_TIMER_INITIAL_COUNT_OFFSET / 4);
    println_serial!("40");
    timer_initial_count.write_volatile(0);
    println_serial!("41");
}


