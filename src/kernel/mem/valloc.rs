use crate::kernel::config::*;

use lib::sync::StaticSpinlock;

use super::PAGE_SIZE;

use core::ptr::NonNull;
use core::ops::Range;

static VALLOC: StaticSpinlock<Option<VAllocator>> = StaticSpinlock::new(None);

struct VAllocator {
    range: Range<NonNull<u8>>,
    cursor: NonNull<u8>
}

impl VAllocator {
    fn new(vrange: Range<*mut u8>) -> VAllocator {
        VAllocator {
            range: NonNull::new(vrange.start).unwrap()..NonNull::new(vrange.end).unwrap(),
            cursor: NonNull::new(vrange.start).unwrap()
        }
    }

    fn alloc(&mut self, page_count: usize) -> core::result::Result<NonNull<u8>, ()> {
        let base_addr = self.cursor.as_ptr();
        let new_cursor = NonNull::new(base_addr.wrapping_add(PAGE_SIZE * page_count)).unwrap();

        if self.range.contains(&new_cursor) {
            self.cursor = new_cursor;
            Ok(NonNull::new(base_addr).unwrap())
        } else {
            Err(())
        }
    }

    fn dealloc(&mut self, _addr: NonNull<u8>, _page_count: usize) {
        // TODO do nothing for now but we will have to write a proper allocator later
    }
}


pub struct VMem {
    base_addr: NonNull<u8>,
    page_count: usize
}

impl VMem {
    pub fn allocate(page_count: usize) -> Result<VMem, ()> {
        if let Some(ref mut allocator) = *VALLOC.lock() {
            Ok(VMem {
                base_addr: allocator.alloc(page_count)?,
                page_count
            })
        } else {
            panic!("vmem has to be initliazed before use");
        }
    }

    pub fn base_addr(&self) -> *mut u8 {
        self.base_addr.as_ptr()
    }

    pub fn page_count(&self) -> usize {
        self.page_count
    }

    pub fn range(&self) -> Range<*mut u8> {
        self.base_addr()
        ..
        self.base_addr().wrapping_add(PAGE_SIZE * self.page_count())
    }

    pub fn leak(vmem: VMem) -> (*mut u8, usize) {
        let leaked = (vmem.base_addr(), vmem.page_count());
        core::mem::forget(vmem);
        leaked
    }

    pub unsafe fn from_raw_parts(base_addr: *mut u8, page_count: usize) -> VMem {
        VMem { base_addr: NonNull::new(base_addr).unwrap(), page_count }
    }
}

impl Drop for VMem {
    fn drop(&mut self) {
        if let Some(ref mut allocator) = *VALLOC.lock() {
            allocator.dealloc(self.base_addr, self.page_count);
        }
    }
}

pub unsafe fn init() {
    let mut allocator = VALLOC.lock();
    if allocator.is_none() {
        let vrange = (VMALLOC_BASE as _)..(VMALLOC_END as _);
        *allocator = Some(VAllocator::new(vrange));
    }
}

