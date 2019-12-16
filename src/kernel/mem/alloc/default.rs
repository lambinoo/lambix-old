use core::alloc::{Layout, GlobalAlloc};
use core::ops::Range;
use core::ptr::NonNull;
use lib::sync::*;

use crate::kernel::mem::addr::*;
use crate::kernel::config::*;
use crate::kernel::mem::paging::{
    PageAllocator,
    PageAllocatorError,
    MemoryPage,
    EarlyAllocator,
    Result,
    map::*
};

static ALLOCATOR: DefaultAllocator<'static> = DefaultAllocator::new();

pub struct DefaultAllocator<'a> {
    inner: Spinlock<Option<InnerAllocator<'a>>>
}

impl<'a> GlobalAlloc for DefaultAllocator<'a> {
    fn allocate() -> Result<Page> {
        Self::lock(|allocator| {
            allocator.alloc()
        })
    }

    fn total_memory() -> usize {
        Self::lock(|allocator| {
            allocator.total_size
        })
    }
}

impl<'a> DefaultAllocator<'a> {
    const fn new() -> DefaultAllocator<'a> {
        let inner = Spinlock::new(None);
        DefaultAllocator { inner }
    }

    fn lock<F,T>(f: F) -> T where F: FnOnce(&mut InnerAllocator) -> T {
        let mut allocator = ALLOCATOR.inner.lock();
        f(allocator.as_mut().unwrap())
    }


    pub unsafe fn init() -> Result<()> {
        *ALLOCATOR.inner.lock() = Some(InnerAllocator::new());

        Self::add_to_memory_pool::<EarlyAllocator>(
            EarlyAllocator::allocate_zeroed()?
            .paddr()
        )?;    

        while let Ok(memory) = EarlyAllocator::allocate() {
            Self::add_to_memory_pool::<DefaultAllocator>(memory.paddr())?;
        }

        let free_memory = Self::allocate()?;

        Ok(())
    }

    pub unsafe fn add_to_memory_pool<MapAlloc: PageAllocator>(paddr: PhyAddr) -> Result<()> {
        let mapped_to = Self::alloc_vaddr::<MapAlloc>()?;
        map4k::<MapAlloc>(paddr, mapped_to, Flags::NO_EXECUTE | Flags::READ_WRITE)
            .expect("failed to map into memory pool");
        Ok(())
    }

    unsafe fn alloc_vaddr<MapAlloc: PageAllocator>() -> Result<VirtAddr> {
        Self::lock(|allocator| {
            allocator.alloc_vaddr::<MapAlloc>()
        })
    }
}


struct InnerAllocator<'a> {
    page_count: usize,
    cursor: VirtAddr,
    allocated_range: Range<VirtAddr>,
    total_size: usize,
    free_memory: Option<NonNull<FreeMemory<'a>>>
}

impl<'a> InnerAllocator<'a> {
    fn new() -> InnerAllocator<'a> {
        let base_addr = Self::vrange().start;
        InnerAllocator {
            page_count: 0,
            cursor: base_addr,
            allocated_range: base_addr..base_addr,
            total_size: 0,
            free_memory: None
        }
    }

    fn alloc(&mut self) -> Result<Page> {
        let mut result = Err(PageAllocatorError::OutOfMemory);
        if self.allocated_range.contains(&self.cursor) {
            result = Ok(Page {
                vaddr: self.cursor
            });

            self.cursor = self.cursor.wrapping_add(Page::PAGE_SIZE);
        }
        result
    }

    unsafe fn alloc_vaddr<MapAlloc: PageAllocator>(&mut self) -> Result<VirtAddr> {
        let mapped_to = self.allocated_range.end;
        if Self::vrange().contains(&mapped_to) {
            self.allocated_range.end = mapped_to.wrapping_add(Page::PAGE_SIZE);
            self.page_count += 1;
            self.total_size += Page::PAGE_SIZE;
            Ok(mapped_to)
        } else {
            Err(PageAllocatorError::OutOfVirtualMemory)
        }
    }

    fn vrange() -> Range<VirtAddr> {
        Range {
            start: VirtAddr::from(PHYSICAL_MEMORY_MAPPING_BASE),
            end: VirtAddr::from(PHYSICAL_MEMORY_MAPPING_END)
        }
    }
}


type MemoryTypeNext<'a> = Option<NonNull<FreeMemory<'a>>>;
#[repr(align(4096))]
struct FreeMemory<'a> {
    prev_block: MemoryTypeNext<'a>,
    next_block: MemoryTypeNext<'a>,
    range: [
        Range<VirtAddr>;
        Page::PAGE_SIZE
    ],
    _phantom: core::marker::PhantomData<&'a FreeMemory<'a>>
}


#[derive(Debug)]
pub struct Page {
    vaddr: VirtAddr
}

impl MemoryPage for Page {
    const PAGE_SIZE: usize = 4096;

    fn vaddr(&self) -> VirtAddr { self.vaddr }
    fn paddr(&self) -> PhyAddr {
        unsafe {
            virt_to_phy_addr(self.vaddr()).expect("this is a bug, not mapped")
        }
    }

    unsafe fn from_paddr(_paddr: PhyAddr) -> Option<Page> { None }
}

