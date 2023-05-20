use crate::kernel::mem::addr::*;
use core::fmt;
use core::mem::{size_of, transmute};
use core::slice;

#[repr(C)]
struct MemoryMapHeader {
    entry_size: u32,
    entry_version: u32,
}

#[repr(transparent)]
pub struct MemoryMap {
    entries: [Memory],
}

impl MemoryMap {
    pub fn from_bytes(bytes: &[u8]) -> Option<&MemoryMap> {
        if bytes.len() >= size_of::<MemoryMapHeader>() {
            let header_ptr = bytes.as_ptr() as *const MemoryMapHeader;
            let entries_ptr = header_ptr.wrapping_add(1) as *const Memory;

            let entries_size = bytes.len() - size_of::<MemoryMapHeader>();
            let entry_size = size_of::<Memory>();

            if entries_size % entry_size == 0 {
                Some(unsafe {
                    transmute(slice::from_raw_parts(
                        entries_ptr,
                        entries_size / entries_size,
                    ))
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn entries(&self) -> &[Memory] {
        &self.entries
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Memory {
    pub base_addr: PhyAddr,
    pub length: u64,
    pub mem_type: MemoryType,
    _reserved: u32,
}

#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum MemoryType {
    #[doc(hidden)]
    __Reserved = 0,
    AvailableRAM = 1,
    ACPIInformation = 3,
    ReservedToPreserve = 4,
    DefectiveRAM = 5,
}

impl fmt::Debug for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MemoryType::AvailableRAM => "AvailableRAM",
            MemoryType::ACPIInformation => "ACPIInformation",
            MemoryType::ReservedToPreserve => "ReservedToPreserve",
            MemoryType::DefectiveRAM => "DefectiveRAM",
            _ => "Reserved",
        };

        write!(f, "{}", name)
    }
}
