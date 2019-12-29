use crate::kernel::config::*;
use crate::kernel::table::paging::*;
use crate::kernel::mem::paging::*;
use crate::kernel::kernel_range;
use crate::boot::multiboot::{memmap::*, *};
use crate::kernel::mem::addr::*;

use core::convert::TryFrom;
use core::alloc::{GlobalAlloc, Layout};
use core::ops::Range;
use lib::sync::*;

#[global_allocator]
static LALLOC: LambixAllocator = LambixAllocator::new();

pub fn init() {
    LALLOC.init();
}

struct LambixAllocator {
    inner: Spinlock<Option<InnerAllocator>>
}

unsafe impl GlobalAlloc for LambixAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Some(ref mut allocator) = *self.inner.lock() {
            allocator.alloc(layout)
        } else {
            panic!("LambixAllocator needs to be initliazed before use");
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if let Some(ref mut allocator) = *self.inner.lock() {
            allocator.dealloc(ptr, layout);
        } else {
            panic!("LambixAllocator needs to be initliazed before use");
        }
    }
}

impl LambixAllocator {
    const fn new() -> LambixAllocator {
        LambixAllocator {
            inner: Spinlock::new(None)
        }
    }

    fn init(&self) {
        let mut available_memory = available_memory_iter();
        let mut counter = 0;
        unsafe {
            self.create_empty_allocator();
            self.add_first_page(&mut available_memory);

            let mut end_addr = self.inner.lock().as_ref().unwrap().memory.end;
            while let Some(memory) = available_memory.next() {
                map4k(end_addr, memory, Flags::PRESENT | Flags::READ_WRITE | Flags::NO_EXECUTE)
                    .expect("we shouldn't map over already mapped memory here, something is really wrong");

             
                end_addr = end_addr.wrapping_add(PageTable::PAGE_SIZE);
                self.inc_mem_pool(PageTable::PAGE_SIZE);

                counter += 1;

                if counter == 80895 { 
                    core::ptr::write_volatile(&mut counter as *mut _, counter);
                }
            }

            self.inner.lock().as_mut().unwrap().memory.end = end_addr;
            early_kprintln!("DONE\n{:?}", self.inner.lock().as_ref());
        };

    }

    fn create_empty_allocator(&self) {
        let mut lock = self.inner.lock();
        let base_addr = VirtAddr::from(PHYSICAL_MEMORY_MAPPING_BASE);
        if lock.is_none() {
            *lock = Some(InnerAllocator::new(base_addr..base_addr));
        } else {
            panic!("an allocator already exists");
        }
    }

    unsafe fn add_first_page(&self, available_memory: &mut impl Iterator<Item=PhyAddr>) { 
        let flags = Flags::NO_EXECUTE | Flags::READ_WRITE | Flags::PRESENT;
        let base_addr = VirtAddr::from(PHYSICAL_MEMORY_MAPPING_BASE);

        let error_message = "not enough memory to bootstrap the kernel memory unit";

        let pdpt = available_memory.next().expect(error_message);
        let pdt = available_memory.next().expect(error_message);
        let pt = available_memory.next().expect(error_message);
        let page = available_memory.next().expect(error_message);

        zero_page_table(pdpt);
        zero_page_table(pdt);
        zero_page_table(pt);

        PageTable::get_entry(PageTableType::PML4T, base_addr).set(pdpt, flags);
        PageTable::get_entry(PageTableType::PDPT, base_addr).set(pdt, flags);
        PageTable::get_entry(PageTableType::PDT, base_addr).set(pt, flags);
        PageTable::get_entry(PageTableType::PT, base_addr).set(page, flags);
    
        self.inc_mem_pool(PageTable::PAGE_SIZE);
    }

    fn inc_mem_pool(&self, size: usize) {
        let mut lock = self.inner.lock();
        let mut allocator = lock.as_mut().unwrap();
        allocator.memory.end = allocator.memory.end.wrapping_add(size);
    }
}


#[derive(Debug)]
struct InnerAllocator {
    memory: Range<VirtAddr>,
    cursor: VirtAddr
}

impl InnerAllocator {
    const fn new(memory: Range<VirtAddr>) -> InnerAllocator {
        InnerAllocator { cursor: memory.start, memory }
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let cursor = self.cursor.align_to(layout.align());
        let new_cursor = self.cursor.wrapping_add(layout.size());

        if new_cursor <= self.memory.end {
            self.cursor = new_cursor;
            cursor.as_mut()
        } else {
            core::ptr::null_mut()
        }
    }

    fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // do nothing, just forget about it
    }
}

#[inline]
unsafe fn zero_page_table(page: PhyAddr) {
    core::ptr::write_bytes(page.as_mut(), 0, PageTable::PAGE_SIZE);
}

fn available_memory_iter() -> impl Iterator<Item=PhyAddr> {
    // here, we still have the 1st GB of memory identity-mapped
    let tags = unsafe {
        BOOT_INFO.tags(VirtAddr(BOOT_INFO.paddr().0))
    };

    tags.filter_map(|tag| tag.as_memmap())
        .flat_map(|map| map.entries())
        .filter(|mem| mem.mem_type == MemoryType::AvailableRAM)
        .flat_map(|mem| {
            let mem_section_size = usize::try_from(mem.length).unwrap();
            let end_addr = mem.base_addr.wrapping_add(mem_section_size) &! PageTable::PAGE_MASK; 
            let start_addr = {
                let addr = mem.base_addr.align::<PageTable>();
                if addr < end_addr {
                    Some(addr)
                } else {
                    None
                }
            };

            core::iter::successors(start_addr, move |addr| {
                let next_base_page = addr.wrapping_add(PageTable::PAGE_SIZE);
                if next_base_page < end_addr {
                    Some(next_base_page)
                } else {
                    None
                }
            })
        })
        .filter(|p| {
            let page = *p .. p.wrapping_add(PageTable::PAGE_SIZE);
            let kernel_range = kernel_range();
            let boot_info = BOOT_INFO.range();
            ((page.start >= boot_info.end) || (page.end <= boot_info.start)) 
                && ((page.start >= kernel_range.end) || (page.end <= kernel_range.start))
        })
}

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("OOM: failed to allocate {:?}", layout)
}

