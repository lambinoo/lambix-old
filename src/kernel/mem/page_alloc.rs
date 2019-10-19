//use crate::kernel::table::paging::*;
use core::ops::Range;
use crate::boot::multiboot::*;
use crate::kernel::mem::addr::*;
use core::sync::atomic::*;
use core::fmt;
use core::mem::MaybeUninit;
use core::marker::PhantomData;
use lib::sync::*;

static INITIALIZED: AtomicBool = AtomicBool::new(false);

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_ADDR_MASK: usize = !(PAGE_SIZE - 1);

static ALLOC_PAGE_DIRECTORY: Mutex<MaybeUninit<AllocatedPageDirectory>> = Mutex::new(MaybeUninit::uninit());

extern {
    static kernel_start_addr: u8;
    static kernel_end_addr: u8; 
}


pub fn init_page_alloc() {
    if INITIALIZED.compare_and_swap(false, true, Ordering::AcqRel) {
        panic!("page_alloc already initialized")
    }
    
    let mem_map = BOOT_INFO.tags()
        .filter(|t| t.tag_type == TagType::MemMap)
        .next().unwrap().as_memmap().unwrap();

    let mut ram_entries = mem_map
        .entries()
        .filter(|m| m.mem_type == memmap::MemoryType::AvailableRAM);

    let mut apd_guard = ALLOC_PAGE_DIRECTORY.lock();
    let apd = unsafe { &mut *apd_guard.as_mut_ptr() };
 
    for section in apd.mem_sections.iter_mut() {
        *section = if let Some(entry) = ram_entries.next() {
            let base_addr = entry.base_addr.wrapping_add(
                entry.base_addr.align_offset(PAGE_SIZE)
            );

            let end_addr =
                entry.base_addr.wrapping_add(entry.length as usize)
                & PAGE_ADDR_MASK;

            base_addr .. end_addr
        } else {
            PhyAddr::NULL .. PhyAddr::NULL
        };

    }

    if !apd.mem_sections[0].is_empty() {
        apd.next_page = apd.mem_sections[0].start;
        apd.current_section = 0;
    } else {
        panic!("no available memory");
    }

    apd.kernel = unsafe {
        PhyAddr(&kernel_start_addr as _) .. PhyAddr(&kernel_end_addr as _)
    };

    if ram_entries.next().is_some() {
        early_kprintln!("WARNING, too many memory sections, some ram will not be used");
    }
}


pub fn page_alloc<'a>() -> Page<'a> {
    debug_assert!(INITIALIZED.load(Ordering::Acquire) == true);

    let mut apd_guard = ALLOC_PAGE_DIRECTORY.lock();
    let apd = unsafe { &mut *apd_guard.as_mut_ptr() };

    let mut allocated_page_addr = None;
    while allocated_page_addr.is_none() {
        let current_section = &apd.mem_sections[apd.current_section];
        let end_page = apd.next_page.wrapping_add(PAGE_SIZE);

        if current_section.contains(&apd.next_page) && current_section.contains(&end_page) {
            if !apd.kernel.contains(&apd.next_page) && !apd.kernel.contains(&end_page) {
                allocated_page_addr = Some(Page {
                    base_addr: apd.next_page,
                    mapped_to: None,
                    data: unsafe { &mut *(*apd.next_page as *mut [u8; 4096]) },
                    __phantom: PhantomData
                });
            }

            apd.next_page = end_page;
        } else {
            apd.current_section += 1;
            if apd.current_section >= apd.mem_sections.len() {
                panic!("Out of memory {:?}", apd);
            }

            apd.next_page = apd.mem_sections[apd.current_section].start;
        }
    }

    allocated_page_addr.unwrap()
}


#[derive(Debug)]
pub struct AllocatedPageDirectory {
    next_page: PhyAddr,
    current_section: usize,
    mem_sections: [Range<PhyAddr>; 32],
    kernel: Range<PhyAddr>
}


pub struct Page<'a> {
    base_addr: PhyAddr,
    mapped_to: Option<VirtAddr>,
    pub data: *mut [u8; 4096],
    __phantom: PhantomData<&'a u8>
}

impl<'a> Page<'a> {
    pub fn range(&self) -> Range<PhyAddr> {
        self.base_addr .. self.base_addr.wrapping_add(PAGE_SIZE)
    }
}

impl<'a> fmt::Debug for Page<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Page {{ base_addr: {:#?}, end_addr: {:#?}, mapped_to: {:?} }}",
            self.base_addr,
            self.base_addr.wrapping_add(PAGE_SIZE),
            self.mapped_to
        )
    }
}

impl<'a> Drop for Page<'a> {
    fn drop(&mut self) {}
}

