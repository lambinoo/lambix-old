use bitflags::*;
use core::ops::Index;
use core::sync::atomic::*;
use core::mem::transmute;
use crate::kernel::mem::addr::*;

pub const ENTRY_PER_TABLE: usize = 512;

macro_rules! table {
    ($name:ident, $offset:expr) => {
        #[repr(align(4096))]
        pub struct $name {
            table: [Entry<$name>; ENTRY_PER_TABLE]
        }

        impl $name {
            pub const fn new() -> Self {
                let table = unsafe { transmute([0u64; 512]) };
                Self { table }
            }

            pub fn offset_of(vaddr: VirtAddr) -> usize {
                usize::from(vaddr >> usize::from($offset)) & 0x1ffusize
            }
        }

        impl Index<usize> for $name {
            type Output = Entry<$name>;
            fn index(&self, index: usize) -> &Self::Output {
                &self.table[index]
            }
        }

        impl Index<VirtAddr> for $name {
            type Output = Entry<$name>;

            fn index(&self, address: VirtAddr) -> &Self::Output {
                &self.table[Self::offset_of(address)]
            }
        }
    }
}


table!(PageMapLevel4Table, 39usize);
table!(PageDirectoryPointerTable, 30usize);
table!(PageDirectoryTable, 21usize);
table!(PageTable, 12usize);

impl Entry<PageMapLevel4Table> {
    pub unsafe fn as_pdp_table(&self) -> &PageDirectoryPointerTable {
        self.base_addr().to_ref()
    }
}

impl Entry<PageDirectoryPointerTable> {
    pub unsafe fn as_pd_table(&self) -> &PageDirectoryTable {
        self.base_addr().to_ref()
    }
}

impl Entry<PageDirectoryTable> {
    pub unsafe fn as_page_table(&self) -> &PageTable {
        self.base_addr().to_ref()
    }
}


#[repr(transparent)]
#[derive(Default)]
pub struct Entry<T> {
    value: AtomicU64,
    phantom: core::marker::PhantomData<T>
}

impl<T> Entry<T> {
    fn new() -> Entry<T> {
        Entry {
            value: AtomicU64::new(0),
            phantom: core::marker::PhantomData
        }
    }

    pub fn get_value(&self) -> u64 {
        self.value.load(Ordering::Acquire)
    }

    pub fn set_value(&self, value: u64) {
        self.value.store(value, Ordering::Relaxed);
    }

    pub fn get_flags(&self) -> Flags {
        let mut flags = Flags::default();
        flags.bits = self.get_value();
        flags
    }

    pub fn base_addr(&self) -> PhyAddr {
        let addr = self.get_value() &! (0xfff | (0xfff << 52));
        PhyAddr::from(addr)
    }

    pub fn is_present(&self) -> bool {
        self.get_flags().contains(Flags::PRESENT)
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Flags : u64 { 
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

