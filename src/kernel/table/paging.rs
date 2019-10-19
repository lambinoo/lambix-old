use bitflags::*;
use core::ops::Index;
use core::marker::PhantomData;
use core::sync::atomic::*;
use core::mem::transmute;
use crate::kernel::mem::{page_alloc::*, addr::*};


#[repr(transparent)]
#[derive(Default)]
pub struct Entry<T> {
    value: AtomicU64,
    __phantom: core::marker::PhantomData<T>
}

impl<T> Entry<T> { 
    fn new() -> Entry<T> {
        Entry { value: AtomicU64::new(0), __phantom: PhantomData }
    }

    pub fn get_value(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }

    pub fn set_value(&self, value: u64) {
        self.value.store(value, Ordering::Relaxed);
    }

    pub fn get_flags(&self) -> Flags {
        let mut flags = Flags::default();
        flags.bits = self.get_value();
        flags
    }
}


#[repr(align(4096))]
pub struct PageTable<'t> {
    table: [Entry<Self>; 512],
    __phantom: PhantomData<&'t u8>
}


#[repr(align(4096))]
pub struct PageDirectoryTable<'t> {
    table: [Entry<Self>; 512],
    __phantom: PhantomData<PageTable<'t>>,
    __phantom2: PhantomData<&'t u8>
}


#[repr(align(4096))]
pub struct PageDirectoryPointerTable<'t> {
    table: [Entry<Self>; 512],
    __phantom: PhantomData<PageDirectoryTable<'t>>,
    __phantom2: PhantomData<&'t u8>
}


#[repr(align(4096))]
pub struct PageMapLevel4Table<'t> {
    table: [Entry<PageMapLevel4Table<'t>>; 512],
    __phantom: PhantomData<PageDirectoryPointerTable<'t>>
}


impl<'t> PageMapLevel4Table<'t> {
    pub const fn new() -> PageMapLevel4Table<'t> {
        

        let table = unsafe { transmute([0u64; 512]) };
        PageMapLevel4Table { table, __phantom: PhantomData }
    }
}


impl<'t> Index<VirtAddr> for PageMapLevel4Table<'t> {
    type Output = Entry<Self>;

    fn index(&self, address: VirtAddr) -> &Self::Output {
        &self.table[usize::from(address >> 39usize & 0x1ffusize)]
    }
}


bitflags! {
    #[derive(Default)]
    pub struct Flags : u64 { 
        const NOT_EXECUTE = 1 << 63;
        const PAGE_ATTR_PDIR = 1 << 12;
        const GLOBAL = 1 << 8;
        const PAGE_SIZE_OR_ATTR = 1 << 7;
        const DIRTY = 1 << 6;
        const ACCESSED = 1 << 5;
        const CACHE_DISABLE = 1 << 4;
        const WRITETHROUGH = 1 << 3;
        const ALLOW_USER = 1 << 2;
        const READ_WRITE = 1 << 1;
        const PRESENT = 1;
        const AVAILABLE_LOCK = 1 << 11;
    }
}

