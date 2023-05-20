use super::vbuffer::VBuffer;
use crate::kernel::mem::addr::*;
use crate::kernel::mem::paging::*;
use alloc::boxed::Box;

pub use crate::kernel::mem::paging::Flags;

use core::mem::ManuallyDrop;

/// Map a physical address to the virtual address space. This is highly unsafe as it can lead to manipulating memory in-use by other part of the kernel
/// on this or another CPU.
/// It can also lead to unaligned read of memory if you are not careful.
///
/// Size of the structure that will be mapped have to be a multiple of the page size.
pub struct VBox<T> {
    inner_box: ManuallyDrop<Box<T>>,
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
    pub unsafe fn new(paddr: PhyAddr) -> Result<VBox<T>> {
        VBox::with_flags(
            paddr,
            Flags::READ_WRITE | Flags::NO_EXECUTE | Flags::CACHE_DISABLE | Flags::WRITETHROUGH,
        )
    }

    /**
     * Map a physical address with custom flags. The PRESENT flag is always implied.
     *
     * # Safety
     * Check the documentation for [`VBox::new`] for more information about safety.
     *
     * [`VBox::new`]: struct.VBox.html#method.new
     **/
    pub unsafe fn with_flags(paddr: PhyAddr, flags: Flags) -> Result<VBox<T>> {
        let vbuffer = VBuffer::with_flags(paddr, core::mem::size_of::<T>(), flags)?;
        Ok(VBox {
            inner_box: ManuallyDrop::new(Box::from_raw(VBuffer::leak(vbuffer).0 as _)),
        })
    }

    /**
     * Leaks a reference to the content of the VBox.
     * Destructor will therefore not be run
     **/
    pub fn leak<'a>(vb: VBox<T>) -> &'a mut T {
        unsafe { &mut *VBox::into_raw(vb) }
    }

    pub fn into_raw(mut vb: VBox<T>) -> *mut T {
        let addr = vb.inner_box.as_mut() as _;
        core::mem::forget(vb);
        addr
    }

    pub unsafe fn from_raw(addr: *mut T) -> VBox<T> {
        VBox {
            inner_box: ManuallyDrop::new(Box::from_raw(addr)),
        }
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
        };

        let vbuffer = unsafe {
            VBuffer::from_raw(base_addr as _, core::mem::size_of::<T>());
        };
        core::mem::drop(vbuffer);
    }
}
