use crate::utils::locked::Locked;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::ptr::NonNull;

struct ListNode {
    next: Option<&'static mut ListNode>,
}

const BLOCK_SIZES: &[usize] = &[8, 16, 32, 64, 128, 256, 512, 1024, 2048];

pub struct FixedSizeBlockAllocator {
    list_heads: [Option<&'static mut ListNode>; BLOCK_SIZES.len()],
    fallback_allocator: linked_list_allocator::Heap,
}

impl FixedSizeBlockAllocator {
    pub const fn new() -> Self {
        const EMPTY: Option<&'static mut ListNode> = None;

        Self {
            list_heads: [EMPTY; BLOCK_SIZES.len()],
            fallback_allocator: linked_list_allocator::Heap::empty(),
        }
    }

    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.fallback_allocator.init(heap_start, heap_size);
    }

    fn fallback_allocate(&mut self, layout: Layout) -> *mut u8 {
        match self.fallback_allocator.allocate_first_fit(layout) {
            Ok(pointer) => pointer.as_ptr(),
            Err(_) => ptr::null_mut(),
        }
    }
}

impl Default for FixedSizeBlockAllocator {
    fn default() -> Self {
        Self::new()
    }
}

unsafe impl GlobalAlloc for Locked<FixedSizeBlockAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        match list_index(&layout) {
            None => allocator.fallback_allocate(layout),
            Some(index) => match allocator.list_heads[index].take() {
                None => {
                    let block_size = BLOCK_SIZES[index];
                    let block_alignment = block_size;
                    let layout = Layout::from_size_align(block_size, block_alignment).unwrap();
                    allocator.fallback_allocate(layout)
                }
                Some(node) => {
                    allocator.list_heads[index] = node.next.take();
                    node as *mut ListNode as *mut u8
                }
            },
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut allocator = self.lock();
        match list_index(&layout) {
            None => {
                let pointer = NonNull::new(ptr).unwrap();
                allocator.fallback_allocator.deallocate(pointer, layout);
            }
            Some(index) => {
                let new_node = ListNode {
                    next: allocator.list_heads[index].take(),
                };
                assert!(size_of::<ListNode>() <= BLOCK_SIZES[index]);
                assert!(align_of::<ListNode>() <= BLOCK_SIZES[index]);
                let new_node_pointer = ptr as *mut ListNode;
                new_node_pointer.write(new_node);
                allocator.list_heads[index] = Some(&mut *new_node_pointer)
            }
        }
    }
}

fn list_index(layout: &Layout) -> Option<usize> {
    let required_block_size = layout.size().max(layout.align());
    BLOCK_SIZES
        .iter()
        .position(|&size| size >= required_block_size)
}
