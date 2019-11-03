use crate::kernel::mem::addr::*;
use core::fmt;

pub struct Page {
    paddr: PhyAddr,
    vaddr: VirtAddr
}

impl<'a> Page {
    const SIZE: usize = 4096;
}

impl<'a> fmt::Debug for Page {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Page {{ paddr: {:#?}, vaddr: {:#?} }}",
            self.paddr.as_ptr(),
            self.vaddr.as_ptr()
        )
    }
}

impl<'a> Drop for Page {
    fn drop(&mut self) {}
}

