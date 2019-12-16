use bitflags::*;
use core::ops::Index;
use core::sync::atomic::*;
use crate::kernel::mem::addr::*;
use crate::kernel::config::*;

/// An entry from a page translation table providing easy access to atomic read & write
#[repr(C)]
pub struct Entry {
    value: AtomicUsize
}

impl Entry {
    /// Get the value of the page translation entry atomically
    ///
    /// # Safety
    /// It's mandatory that scheduling & interrupts be disabled when calling this function
    pub unsafe fn get_value(&self) -> usize {
        self.value.load(Ordering::Release)
    }

    /// Set the value in the entry atomically
    ///
    /// # Safety
    /// It's mandatory that scheduling & interrupts be disabled when calling this function.
    ///
    /// `value` has to be a correct page translation table entry (cf. amd64 sys programming doc).
    ///
    /// This function will also change the current state of the pagination table, meaning the
    /// address space will change.
    pub unsafe fn set_value(&self, value: usize) {
        self.value.store(value, Ordering::Acquire);
    }
}


/// A general representation of a page translation table, giving access to it's entries atomically.
/// Every table is made of `ENTRY_PER_TABLE` 
#[repr(align(4096))]
pub struct PageTable {
    entries: [Entry; PageTable::ENTRY_COUNT]
}

impl PageTable {
    const IDX_BITS: usize = 9;
    const IDX_MASK: usize = (1 << Self::IDX_BITS) - 1;

    const PAGE_BITS: usize = 12;
    const PAGE_MASK: usize = (1 << Self::PAGE_BITS) - 1;

    pub const ENTRY_COUNT: usize = 1 << Self::IDX_BITS;
    pub const PAGE_SIZE: usize   = 1 << Self::PAGE_BITS;

    /// Return the table of type `table_type` associated with the address `addr`
    ///
    /// # Safety
    /// This may create a reference towards a PageTable that doesn't exist. You have to ensure that
    /// it does in fact exist or a page fault will probably be raised on access.
    ///
    /// You have to ensure that the scheduler & interrupts are disabled while accessing the page
    /// table or the page table might not be valid.
    pub unsafe fn get_table<'a>(table_type: PageTableType, addr: VirtAddr) -> &'a PageTable {
        Self::get_table_addr(table_type, addr).to_ref::<PageTable>()
    }

    /// Get the index in a page table of level `level` for the `addr` address
    #[inline]
    pub const fn get_index(table_type: PageTableType, addr: VirtAddr) -> usize {
        usize::from(addr >> Self::get_shift(table_type)) & Self::IDX_MASK
    }
}

#[doc(hidden)]
impl PageTable {
    #[inline]
    const fn get_shift(table_type: PageTableType) -> usize {
        Self::PAGE_BITS + Self::IDX_BITS * (table_type as usize)
    }

    #[inline]
    fn table_addr(pdpt: usize, pdt: usize, pt: usize) -> VirtAddr {
        VirtAddr::from(PAGE_MAP_BASE)
            | (pdpt << Self::get_shift(PageTableType::PDPT))
            | (pdt  << Self::get_shift(PageTableType::PDT))
            | (pt   << Self::get_shift(PageTableType::PT))
    }

    fn get_table_addr(table_type: PageTableType, addr: VirtAddr) -> VirtAddr {
        let root_idx = Self::get_index(PageTableType::PML4T, VirtAddr::from(PAGE_MAP_BASE));
        match table_type {
            PageTableType::PML4T => Self::table_addr(root_idx, root_idx, root_idx),

            PageTableType::PDPT => {
                Self::table_addr(
                    root_idx,
                    root_idx,
                    Self::get_index(PageTableType::PDPT, addr)
                )
            },

            PageTableType::PDT => {
                Self::table_addr(
                    root_idx,
                    Self::get_index(PageTableType::PDPT, addr),
                    Self::get_index(PageTableType::PDT, addr)
                )
            },

            PageTableType::PT => {
                Self::table_addr(
                    Self::get_index(PageTableType::PDPT, addr),
                    Self::get_index(PageTableType::PDT, addr),
                    Self::get_index(PageTableType::PT, addr)
                )
            }
        }
    }

}

impl Index<usize> for PageTable {
    type Output = Entry;
    fn index(&self, idx: usize) -> &Entry {
        &self.entries[idx]
    }
}


/// Different types of page translation tables
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum PageTableType {
    /// PageMap Level-4 Table
    PML4T = 3,
    /// Page Directory Pointer Table 
    PDPT = 2,
    /// Page Directory Table
    PDT = 1,
    /// Page Table
    PT = 0,
}

bitflags! {
    /// List of flags available in a page translation table entry
    #[derive(Default)]
    pub struct Flags : usize { 
        const NO_EXECUTE = 1 << 63;
        const PAGE_ATTR = 1 << 12;
        const GLOBAL = 1 << 8;
        const PAGE_SIZE = 1 << 7;
        const PAGE_ATTR_PTE = 1 << 7;
        const DIRTY = 1 << 6;
        const ACCESSED = 1 << 5;
        const CACHE_DISABLE = 1 << 4;
        const WRITETHROUGH = 1 << 3;
        const ALLOW_USER = 1 << 2;
        const READ_WRITE = 1 << 1;
        const PRESENT = 1;
    }
}

