use buddy_system_allocator::LockedHeap;

const MIN_BLK_SIZE: usize = 32;

#[global_allocator]
static KERNEL_ALLOCATOR: LockedHeap<MIN_BLK_SIZE> = LockedHeap::empty();

extern "C" {
    static heap_begin: u8;
    static heap_end: u8;
}

pub fn init() {
    let base_address: usize = unsafe { &heap_begin as *const u8 as usize };
    let end_address: usize = unsafe { &heap_end as *const u8 as usize };

    unsafe {
        KERNEL_ALLOCATOR
            .lock()
            .add_to_heap(base_address, end_address);
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
