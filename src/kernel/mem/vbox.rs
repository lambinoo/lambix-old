use lib::sync::*;

use crate::kernel::mem::paging::*;
use crate::kernel::mem::addr::*;
use crate::kernel::config::*;
use alloc::alloc::Layout;
use alloc::boxed::Box;

pub use crate::kernel::mem::paging::Flags;

use core::ops::Range;
use core::ptr::NonNull;
use core::mem::ManuallyDrop;

static ALLOCATOR: Spinlock<Option<VAllocator>> = Spinlock::new(None);

/// Map a physical address to the virtual address space. This is highly unsafe as it can lead to manipulating memory in-use by other part of the kernel
/// on this or another CPU.
/// It can also lead to unaligned read of memory if you are not careful.
///
/// Size of the structure that will be mapped have to be a multiple of the page size.
#[derive(Debug)]
pub struct VBox<T> {
    inner_box: ManuallyDrop<Box<T>>
}

impl<T> VBox<T> {
    /**
     * Map a physical address to the virtual address space with default flags.
     * Those flags are [`PRESENT`], [`READ_WRITE`], [`NO_EXECUTE`],  [`CACHE_DISABLE`] and [`WRITETHROUGH`].
     *
     * # Safety
     * You have to be careful which physical address you map. You could access memory that is used
     * somewhere else in the code and/or by another CPU.
     *
     * [`PRESENT`]: ../../table/paging/struct.Flags.html#associatedconstant.PRESENTcar
     * [`READ_WRITE`]: ../../table/paging/struct.Flags.html#associatedconstant.READ_WRITE
     * [`NO_EXECUTE`]: ../../table/paging/struct.Flags.html#associatedconstant.NO_EXECUTE
     * [`CACHE_DISABLE`]: ../../table/paging/struct.Flags.html#associatedconstant.CACHE_DISABLE
     * [`WRITETHROUGH`]: ../../table/paging/struct.Flags.html#associatedconstant.WRITETHROUGH
     **/
    pub unsafe fn new(paddr: PhyAddr) -> VBox<T> {
        VBox::with_flags(paddr, Flags::READ_WRITE | Flags::NO_EXECUTE | Flags::CACHE_DISABLE | Flags::WRITETHROUGH)
    }

    /**
     * Map a physical address with custom flags. The PRESENT flag is always implied.
     *
     * # Safety
     * Check the documentation for [`VBox::new`] for more information about safety.
     *
     * [`VBox::new`]: struct.VBox.html#method.new
     **/
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
                inner_box: ManuallyDrop::new(Box::from_raw(base_addr as *mut T))
            }
        } else {
            panic!("VBox framework wasn't initialized before use");
        }
    }

    /**
     * Leaks a reference to the content of the VBox.
     * Destructor will therefore not be run
     **/
    pub fn leak<'a>(vb: VBox<T>) -> &'a mut T {
        unsafe { &mut *VBox::into_raw(vb) }
    }

    pub fn into_raw(mut vb: VBox<T>) -> *mut T {
        let ptr = vb.inner_box.as_mut() as *mut T;
        core::mem::forget(vb);
        ptr
    }

    pub unsafe fn from_raw(p: *mut T) -> VBox<T> {
        VBox {
            inner_box: ManuallyDrop::new(Box::from_raw(p))
        } 
    }

    #[inline]
    fn layout() -> Layout {
        Layout::new::<T>().align_to(PAGE_SIZE).expect("this type can't be accessed through a VBox")
    }
}

impl<T> core::ops::Deref for VBox<T> {
    type Target = Box<T>;
    fn deref(&self) -> &Box<T> {
        &self.inner_box
    }
}

impl<T> core::ops::DerefMut for VBox<T> {
    fn deref_mut(&mut self) -> &mut Box<T> {
        &mut self.inner_box
    }
}


impl<T> Drop for VBox<T> {
    fn drop(&mut self) {
        let base_addr = self.inner_box.as_mut() as *mut T;
        unsafe {
            core::ptr::drop_in_place(base_addr);
            unmap4k(VirtAddr::from(base_addr)).expect("unsound state, already allocated vmem not mapped");
        };

        if let Some(ref mut allocator) = *ALLOCATOR.lock() {
            allocator.dealloc(Self::layout(), base_addr as *mut u8);
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

