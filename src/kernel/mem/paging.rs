pub type PhysicalAddress = *const u8;
pub type VirtualAddress = *const u8;

bitflags! {
    struct Flags : u64 { 
        const NOT_EXECUTE = 1 << 63;
        const PAGE_ATTR_PTAB = 1 << 7;
        const PAGE_ATTR_PDIR = 1 << 12;
        const GLOBAL = 1 << 8;
        const PAGE_SIZE = 1 << 7;
        const DIRTY = 1 << 6;
        const ACCESSED = 1 << 5;
        const CACHE_DISABLE = 1 << 4;
        const WRITETHROUGH = 1 << 3;
        const ALLOW_USER = 1 << 2;
        const READ_WRITE = 1 << 1;
        const PRESENT = 1;
    }
}

#[repr(C)]
struct PageTable {

}

#[repr(C)]
struct PageDirectoryTable {

}

#[repr(C)]
struct PageDirectoryPointerTable {

}

#[repr(C)]
struct PageMapLevel4Table {

}

