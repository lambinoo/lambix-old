use core::alloc::GlobalAlloc;
use core::ops::Range;
use core::convert::TryFrom;
use crate::kernel::mem::addr::*;
use crate::boot::multiboot::*;
use crate::kernel::mem::addr::*;
use crate::kernel::paging::{PageAllocator, PageAllocatorError, MemoryPage};
use lib::sync::*;




static EARLY_ALLOCATOR: EarlyAllocator = EarlyAllocator {
    inner: Spinlock::new(None)
};

pub struct EarlyAllocator {
    inner: Spinlock<Option<InnerAllocator>>
}

impl GlobalAlloc for EarlyAllocator {
    fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = EARLY_ALLOCATOR.inner.lock();
        if allocator.is_none() {
            *allocator = Some(InnerAllocator::new());
        }

        allocator.as_mut().unwrap().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // no-op
    }
}


#[derive(Debug)]
struct InnerAllocator {
    next_addr: VirtAddr,
    current_section: usize,
    mem_sections: [Range<PhyAddr>; 32],
    kernel: Range<PhyAddr>
}

impl InnerAllocator {
    fn alloc(&mut self) -> VirtAddr {
        let mut allocated_page = VirtAddr::NULL;
        while allocated_page.is_err() {
            let current_section = &self.mem_sections[self.current_section];
            let end_page = self.next_page.wrapping_add(Self::PAGE_SIZE);

            if current_section.contains(&self.next_page) && current_section.contains(&end_page) {
                if !self.kernel.contains(&self.next_page) && !self.kernel.contains(&end_page) {
                    allocated_page = Ok(EarlyAllocatorPage::new(self.next_page));
                }
                self.next_page = end_page;
            } else {
                self.current_section += 1;
                if self.current_section < self.mem_sections.len() {
                    self.next_page = self.mem_sections[self.current_section].start;
                } else {
                    break;
                }
            }
        }
        
        allocated_page
    }

    unsafe fn init(&mut self) {
        let mut ram_entries = BOOT_INFO.tags()
            .filter_map(|t| t.as_memmap())
            .flat_map(|m| m.entries())
            .filter(|m| m.mem_type == memmap::MemoryType::AvailableRAM);

        for section in self.mem_sections.iter_mut() {
            *section = if let Some(entry) = ram_entries.next() {
                let base_addr = entry.base_addr.wrapping_add(
                    entry.base_addr.as_ptr().align_offset(Self::PAGE_SIZE)
                );

                let end_addr =
                    entry.base_addr.wrapping_add(usize::try_from(entry.length).unwrap())
                    & !(Self::PAGE_SIZE - 1);

                early_kprintln!("{:?}", base_addr..end_addr);

                base_addr .. end_addr
            } else {
                PhyAddr::NULL .. PhyAddr::NULL
            };
        }        

        if !self.mem_sections[0].is_empty() {
            self.next_page = self.mem_sections[0].start;
            self.current_section = 0;
        } else {
            panic!("no available memory");
        }

        self.kernel = PhyAddr(&kernel_start_addr as _) .. PhyAddr(&kernel_end_addr as _);

        if ram_entries.next().is_some() {
            early_kprintln!("WARNING, too many memory sections, some ram will not be used");
        }
    }

    fn new() -> InnerAllocator {
        unsafe {
            let mut allocator: InnerAllocator = core::mem::MaybeUninit::zeroed().assume_init();
            allocator.init();
            allocator
        }
    }
}

#[derive(Debug)]
pub struct EarlyAllocatorPage {
    paddr: PhyAddr
}

impl EarlyAllocatorPage {
    fn new(paddr: PhyAddr) -> Self {
        Self { paddr }
    }
}

impl MemoryPage for EarlyAllocatorPage {
    const PAGE_SIZE: usize = 4096;

    fn paddr(&self) -> PhyAddr {
        self.paddr
    }

    fn vaddr(&self) -> VirtAddr {
        VirtAddr::from(usize::from(self.paddr))
    }

    unsafe fn from_paddr(paddr: PhyAddr) -> Option<EarlyAllocatorPage> {
        Some(Self::new(paddr))
    }
}

