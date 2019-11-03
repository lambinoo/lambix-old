mod early_alloc;
mod map;

use crate::kernel::mem::addr::*;
use core::fmt::Debug;

pub type Result<T> = core::result::Result<T, PageAllocatorError>;

pub trait PageAllocator : Sync {
    type Page: MemoryPage;

    fn allocate() -> Result<Self::Page>;
    fn allocate_zeroed() -> Result<Self::Page> {
        let page = Self::allocate();
        if let Ok(ref p) = page {
            unsafe {
                core::ptr::write_bytes(p.vaddr().as_mut(), 0, Self::Page::PAGE_SIZE);
            }
        }

        page
    }
}


#[derive(Debug)]
pub enum PageAllocatorError {
    OutOfMemory
}

pub trait MemoryPage: Debug + Sized {
    const PAGE_SIZE: usize;

    fn paddr(&self) -> PhyAddr;
    fn vaddr(&self) -> VirtAddr;
    unsafe fn from_paddr(paddr: PhyAddr) -> Option<Self>;
}


pub fn init() {
    map::init();
}

