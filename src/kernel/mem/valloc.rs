use crate::kernel::config::*;

use lib::sync::StaticSpinlock;
use alloc::alloc::Layout;

use core::ptr::NonNull;
use core::ops::Range;

pub static VALLOC: StaticSpinlock<Option<VAllocator>> = StaticSpinlock::new(None);

pub struct VAllocator {
    range: Range<NonNull<u8>>,
    cursor: NonNull<u8>
}

impl VAllocator {
    pub fn new(vrange: Range<*mut u8>) -> VAllocator {
        VAllocator {
            range: NonNull::new(vrange.start).unwrap()..NonNull::new(vrange.end).unwrap(),
            cursor: NonNull::new(vrange.start).unwrap()
        }
    }

    pub fn alloc(&mut self, layout: Layout) -> core::result::Result<NonNull<u8>, ()> {
        let base_addr = self.cursor.as_ptr()
            .wrapping_add(self.cursor.as_ptr().align_offset(layout.align()));

        let new_cursor = NonNull::new(base_addr.wrapping_add(layout.size())).unwrap();

        if self.range.contains(&new_cursor) {
            self.cursor = new_cursor;
            Ok(NonNull::new(base_addr).unwrap())
        } else {
            Err(())
        }
    }

    pub fn dealloc(&mut self, _layout: Layout, _ptr: *mut u8) {
        // TODO do nothing for now but we will have to write a proper allocator later
    }
}


pub unsafe fn init() {
    let mut allocator = VALLOC.lock();
    if allocator.is_none() {
        let vrange = (VMALLOC_BASE as _)..(VMALLOC_END as _);
        *allocator = Some(VAllocator::new(vrange));
    }
}

