use lib::sync::*;

use crate::kernel::mem::paging::*;
use crate::kernel::mem::addr::*;
use crate::kernel::config::*;
use alloc::alloc::Layout;

use core::ops::Range;
use core::ptr::NonNull;

static ALLOCATOR: Spinlock<Option<VAllocator>> = Spinlock::new(None);

#[derive(Debug)]
pub struct VBox<T> {
    base_addr: NonNull<T>,
    paddr: PhyAddr
}

impl<T> VBox<T> {
    pub unsafe fn new(paddr: PhyAddr) -> VBox<T> {
        VBox::with_flags(paddr, Flags::READ_WRITE | Flags::NO_EXECUTE | Flags::CACHE_DISABLE)
    }

    pub unsafe fn with_flags(paddr: PhyAddr, flags: Flags) -> VBox<T> {
        let layout = Self::layout();

        if let Some(ref mut allocator) = *ALLOCATOR.lock() {
            let base_addr = allocator.alloc(layout).expect("failed to allocate virtual memory").as_ptr();

            let mut offset = 0;
            while offset < layout.size() {
                map4k(
                    VirtAddr::from(base_addr.wrapping_add(offset)),
                    paddr.wrapping_add(offset),
                    Flags::PRESENT | flags
                ).expect("failed to map virtual memory for VBox");

                offset += PAGE_SIZE;
            }

            VBox {
                base_addr: NonNull::new(base_addr as *mut _).unwrap(),
                paddr: paddr
            }

        } else {
            panic!("VBox framework wasn't initialized before use");
        }
    }

    #[inline]
    fn layout() -> Layout {
        Layout::new::<T>().align_to(PAGE_SIZE).expect("this type can't be accessed through a VBox")
    }
}

impl<T> Drop for VBox<T> {
    fn drop(&mut self) {
        unsafe {
            unmap4k(VirtAddr::from(self.base_addr.as_ptr()))
                .expect("unsound state, already allocated vmem not mapped");
        };

        if let Some(ref mut allocator) = *ALLOCATOR.lock() {
            allocator.dealloc(Self::layout(), self.base_addr.as_ptr() as *mut _);
        }
    }
}


struct VAllocator {
    range: Range<NonNull<u8>>,
    cursor: NonNull<u8>
}

impl VAllocator {
    fn new() -> VAllocator {
        VAllocator {
            range: NonNull::new(VMALLOC_BASE as _).unwrap()..NonNull::new(VMALLOC_END as _).unwrap(),
            cursor: NonNull::new(VMALLOC_BASE as _).unwrap()
        }
    }

    fn alloc(&mut self, layout: Layout) -> core::result::Result<NonNull<u8>, ()> {
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

    fn dealloc(&mut self, _layout: Layout, _ptr: *mut u8) {
        // TODO do nothing for now but we will have to write a proper allocator later
    }
}


pub unsafe fn init() {
    let mut allocator = ALLOCATOR.lock();
    if allocator.is_none() {
        *allocator = Some(VAllocator::new());
    }
}
