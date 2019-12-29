use bitflags::*;
use core::ops::Index;
use core::sync::atomic::*;
use crate::kernel::mem::addr::*;
use crate::kernel::config::*;
use ::alloc::boxed::Box;

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
        self.value.load(Ordering::Acquire)
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
        self.value.store(value, Ordering::Release);
    }

    /// Helper function to insert an address & OR it with the flags.
    ///
    /// # Safety
    /// `addr` must be a properly masked & `flags` must only contains valid flags for the type of entry
    /// cf. `set_value` for reference.
    pub unsafe fn set(&self, addr: PhyAddr, flags: Flags) {
        self.set_value(usize::from(addr) | flags.bits())
    }

    /// Check if the entry is present
    pub unsafe fn is_present(&self) -> bool {
        Flags::from_bits_truncate(self.get_value()).contains(Flags::PRESENT)
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

    pub const PAGE_MASK: usize = (1 << Self::PAGE_BITS) - 1;
    pub const ENTRY_COUNT: usize = 1 << Self::IDX_BITS;

    /// Size of a normal page
    pub const PAGE_SIZE: usize   = 1 << Self::PAGE_BITS;

    pub fn default_flags() -> Flags {
        Flags::NO_EXECUTE | Flags::READ_WRITE | Flags::PRESENT
    }

    /// Return the table of type `table_type` associated with the address `addr`
    ///
    /// # Safety
    /// This may create a reference towards a PageTable that doesn't exist. You have to ensure that
    /// it does in fact exist or a page fault will probably be raised on access.
    ///
    /// You have to ensure that the scheduler & interrupts are disabled while accessing the page
    /// table or the page table might not be valid.
    pub unsafe fn get_table<'a>(table_type: PageTableType, addr: VirtAddr) -> &'a PageTable {
        let table_addr = Self::get_table_addr(table_type, addr);
        table_addr.to_ref::<PageTable>()
    }

    /// Return the entry from the table of type `table_type` associated with the address `addr`
    ///
    /// # Safety
    /// See `get_table`.
    pub unsafe fn get_entry<'a>(table_type: PageTableType, addr: VirtAddr) -> &'a Entry {
        let idx = Self::get_index(table_type, addr);
        let table = Self::get_table(table_type, addr);
        &table[idx]
    }

    /// Get the index in a page table of level `level` for the `addr` address
    pub fn get_index(table_type: PageTableType, addr: VirtAddr) -> usize {
        usize::from(addr >> Self::get_shift(table_type)) & Self::IDX_MASK
    }

    /// Create new zeroed page table
    pub fn new() -> Box<PageTable> {
        unsafe { Box::new_zeroed().assume_init() }
    }
}

#[doc(hidden)]
impl PageTable {
    fn get_shift(table_type: PageTableType) -> usize {
        Self::PAGE_BITS + Self::IDX_BITS * table_type.get_level()
    }

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
                    Self::get_index(PageTableType::PML4T, addr)
                )
            },

            PageTableType::PDT => {
                Self::table_addr(
                    root_idx,
                    Self::get_index(PageTableType::PML4T, addr),
                    Self::get_index(PageTableType::PDPT, addr)
                )
            },

            PageTableType::PT => {
                Self::table_addr(
                    Self::get_index(PageTableType::PML4T, addr),
                    Self::get_index(PageTableType::PDPT, addr),
                    Self::get_index(PageTableType::PDT, addr)
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
#[derive(Copy, Clone)]
pub enum PageTableType {
    /// PageMap Level-4 Table
    PML4T,
    /// Page Directory Pointer Table 
    PDPT,
    /// Page Directory Table
    PDT,
    /// Page Table
    PT
}

impl PageTableType {
    pub fn get_level(&self) -> usize {
        match self {
            Self::PML4T => 3,
            Self::PDPT => 2,
            Self::PDT => 1,
            Self::PT => 0
        }
    }
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

