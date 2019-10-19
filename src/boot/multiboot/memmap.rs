use crate::kernel::mem::addr::*;
use crate::boot::multiboot::*;
use core::mem::size_of;
use core::ops::Index;
use core::fmt;

#[repr(C)]
#[derive(Debug)]
pub struct MemoryMap {
    tag: Tag, 
    entry_size: u32,
    entry_version: u32
}

impl MemoryMap {
    pub fn len(&self) -> usize {
        let mut total_size = self.tag.size as usize;
        total_size = total_size.wrapping_sub(size_of::<MemoryMap>());
        total_size /= self.entry_size as usize;
        total_size
    }

    pub fn entries(&self) -> MemoryEntries {
        MemoryEntries { index: 0, memory_map: self }
    }
}

impl Index<usize> for MemoryMap {
    type Output = Memory;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.len() {
            let ptr = (self as *const MemoryMap).wrapping_add(1) as *const u8;
            unsafe { &*(ptr.wrapping_add(index * (self.entry_size as usize)) as *const Memory) }
        } else {
            panic!("trying to get not existing memory");
        }
    }
}


pub struct MemoryEntries<'m> {
    index: usize,
    memory_map: &'m MemoryMap
}

impl<'m> Iterator for MemoryEntries<'m> {
    type Item = &'m Memory;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.memory_map.len() {
            let result = Some(&self.memory_map[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct Memory {
    pub base_addr: PhyAddr,
    pub length: u64,
    pub mem_type: MemoryType,
    _reserved: u32
}


#[repr(u32)]
#[derive(Copy, Clone, PartialEq)]
pub enum MemoryType {
    #[doc(hidden)]
    __Reserved = 0,
    AvailableRAM = 1,
    ACPIInformation = 3,
    ReservedToPreserve = 4,
    DefectiveRAM = 5
}

impl fmt::Debug for MemoryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            MemoryType::AvailableRAM => "AvailableRAM",
            MemoryType::ACPIInformation => "ACPIInformation",
            MemoryType::ReservedToPreserve => "ReservedToPreserve",
            MemoryType::DefectiveRAM => "DefectiveRAM",
            _ => "Reserved"
        };

        write!(f, "{}", name)
    }
}

