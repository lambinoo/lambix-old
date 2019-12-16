use core::alloc::{GlobalAlloc, Layout};
use core::ops::Range;
use lib::sync::*;

use crate::kernel::mem::addr::*;

#[global_allocator]
static LALLOC: LambixAllocator = LambixAllocator::new();

pub fn init(memory: Range<VirtAddr>) {
    LALLOC.init(memory);
}

struct LambixAllocator {
    inner: Spinlock<Option<InnerAllocator>>
}

unsafe impl GlobalAlloc for LambixAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(ref allocator) = self.inner.lock() {
            self.alloc(layout)
        } else {
            panic!("LambixAllocator needs to be initliazed before use");
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ref allocator) = self.inner.lock() {
            self.dealloc(ptr, layout);
        } else {
            panic!("LambixAllocator needs to be initliazed before use");
        }
    }
}

impl LambixAllocator {
    fn new() -> LambixAllocator {
        LambixAllocator {
            inner: Spinlock::new(None)
        }
    }

    fn init(&self, memory: Range<VirtAddr>) {
        let mut allocator = self.inner.lock();
        if allocator.is_none() {
            *allocator = Some(InnerAllocator::new(memory));
        } else {
            panic!("LambixAllocator was already initiliazed");
        }
    }
}


struct InnerAllocator {
    memory: Range<VirtAddr>,
    cursor: VirtAddr
}

impl InnerAllocator {
    fn new(memory: Range<VirtAddr>) {
        InnerAllocator { cursor: memory.start, memory }
    }

    fn alloc(&self, layout: Layout) -> *mut u8 {
        core::ptr::null_mut() 
    }

    fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {

    }
}


#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("OOM: failed to allocate {:?}", layout)
}

