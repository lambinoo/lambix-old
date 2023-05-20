use super::valloc::VMem;

use crate::kernel::mem::addr::*;
use crate::kernel::mem::paging::Flags;
use crate::kernel::mem::paging::*;

use alloc::alloc::Layout;

pub struct VBuffer {
    addr: *mut u8,
    size: usize,
}

impl VBuffer {
    pub unsafe fn new(paddr: PhyAddr, size: usize) -> Result<VBuffer> {
        VBuffer::with_flags(paddr, size, Flags::READ_WRITE | Flags::NO_EXECUTE)
    }

    pub unsafe fn with_flags(paddr: PhyAddr, size: usize, flags: Flags) -> Result<VBuffer> {
        let layout = Layout::from_size_align(size, PAGE_SIZE).unwrap();
        let low_mask = !(PAGE_SIZE - 1);

        let base_page_addr = paddr & low_mask;
        let page_end_addr = paddr.wrapping_add(layout.size()).align_to(PAGE_SIZE);

        let page_count =
            usize::from(page_end_addr).wrapping_sub(usize::from(base_page_addr)) / PAGE_SIZE;

        let vmem = VMem::allocate(page_count).expect("out of virtual memory");
        for i in 0..vmem.page_count() {
            let offset = i * PAGE_SIZE;
            map4k(
                VirtAddr::from(vmem.base_addr().wrapping_add(offset)),
                base_page_addr.wrapping_add(offset),
                flags,
            )?
        }

        let addr = VirtAddr::from(VMem::leak(vmem).0);

        Ok(VBuffer {
            addr: (addr | (usize::from(paddr) & !low_mask)).as_mut_ptr(),
            size,
        })
    }

    pub fn as_mut_ptr<T>(&self) -> *mut T {
        self.addr as _
    }

    pub fn as_ptr<T>(&self) -> *const T {
        self.addr as _
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub unsafe fn as_ref<T>(&self) -> &T {
        &*self.as_ptr()
    }

    pub unsafe fn as_mut<T>(&self) -> &mut T {
        &mut *self.as_mut_ptr()
    }

    pub fn leak(vbuf: VBuffer) -> (*mut u8, usize) {
        let ret = (vbuf.as_mut_ptr(), vbuf.size());
        core::mem::forget(vbuf);
        ret
    }

    pub unsafe fn from_raw(addr: *mut u8, size: usize) -> VBuffer {
        VBuffer { addr, size }
    }
}

impl Drop for VBuffer {
    fn drop(&mut self) {
        let low_mask = !(PAGE_SIZE - 1);

        let addr = VirtAddr::from(self.as_ptr::<u8>());

        let base_page_addr = addr & low_mask;
        let last_page_end = addr.align_to(PAGE_SIZE);
        let page_count =
            usize::from(base_page_addr).wrapping_sub(usize::from(last_page_end)) / PAGE_SIZE;

        unsafe {
            core::mem::drop(VMem::from_raw_parts(addr.as_mut_ptr(), page_count));
        }
    }
}
