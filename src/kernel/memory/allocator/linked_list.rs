use crate::kernel::memory::heap_allocator::{align_up, Locked};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
}

impl ListNode {
    const fn new(size: usize) -> Self {
        Self {
            size,
            next: None,
        }
    }

    fn start_address(&self) -> usize {
        self as *const Self as usize
    }

    fn end_address(&self) -> usize {
        self.start_address() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    pub const fn new() -> Self {
        LinkedListAllocator {
            head: ListNode::new(0),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    unsafe fn add_free_region(&mut self, address: usize, size: usize) {
        assert_eq!(align_up(address, align_of::<ListNode>()), address);
        assert!(size >= size_of::<ListNode>());

        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_pointer = address as *mut ListNode;
        node_pointer.write(node);
        self.head.next = Some(&mut *node_pointer);
    }

    fn find_region(&mut self, size: usize, alignment: usize) -> Option<(&'static mut ListNode, usize)> {
        let current = &mut self.head;

        while let Some(ref mut region) = current.next {
            if let Ok(allocation_start) = Self::allocate_from_region(&region, size, alignment) {
                let next = region.next.take();
                let output = Some((current.next.take().unwrap(), allocation_start));
                current.next = next;
                return output;
            }
        }

        None
    }

    fn allocate_from_region(region: &ListNode, size: usize, alignment: usize) -> Result<usize, ()> {
        let allocation_start = align_up(region.start_address(), alignment);
        let allocation_end = allocation_start.checked_add(size).ok_or(())?;

        if allocation_end > region.end_address() {
            return Err(());
        }

        let excess_size = region.end_address() - allocation_end;
        if excess_size > 0 && excess_size < size_of::<ListNode>() {
            return Err(());
        }
        Ok(allocation_start)
    }

    fn size_alignment(layout: Layout) -> (usize, usize) {
        let la = layout
            .align_to(align_of::<ListNode>())
            .expect("Adjusting alignment failed")
            .pad_to_align();
        let size = la.size().max(size_of::<ListNode>());
        (size, layout.align())
    }
}

unsafe impl GlobalAlloc for Locked<LinkedListAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let (size, alignment) = LinkedListAllocator::size_alignment(layout);
        let mut allocator = self.lock();

        if let Some((region, allocation_start)) = allocator.find_region(size, alignment) {
            let allocation_end = allocation_start.checked_add(size).expect("Addition overflow");
            let excess_size = region.end_address() - allocation_end;

            if excess_size > 0 {
                allocator.add_free_region(allocation_end, excess_size);
            }

            return allocation_start as *mut u8;
        }

        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let (size, _) = LinkedListAllocator::size_alignment(layout);

        self.lock().add_free_region(ptr as usize, size);
    }
}