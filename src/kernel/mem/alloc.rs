mod default;
mod linear;

use core::alloc::{GlobalAlloc, Layout};

pub fn init() {}

struct BadLinearAllocator;

unsafe impl GlobalAlloc for BadLinearAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {}
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static ALLOCATOR: BadLinearAllocator = BadLinearAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("OOM: failed to allocate {:?}", layout)
}

