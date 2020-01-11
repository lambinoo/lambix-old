use crate::kernel::mem::addr::*;
use crate::boot::multiboot::*;
use core::mem::size_of;
use core::convert::TryFrom;
use core::ops::Index;
use core::fmt;

#[repr(C)]
pub struct MemoryMap {
    tag: Tag, 
    entry_size: u32,
    entry_version: u32
}

impl MemoryMap {
    pub fn len(&self) -> usize {
        let mut total_size = usize::try_from(self.tag.size).unwrap() ;
        total_size = total_size.wrapping_sub(size_of::<MemoryMap>());
        total_size /= usize::try_from(self.entry_size).unwrap();
        total_size
    }

    pub fn entries(&self) -> MemoryEntries {
        MemoryEntries {
            index: 0,
            memory_map: self,
            bytes_read: size_of::<MemoryMap>()
        }
    }
}

impl Index<usize> for MemoryMap {
    type Output = Memory;

    fn index(&self, index: usize) -> &Self::Output {
        if index < self.len() {
            let ptr = VirtAddr::from(self).wrapping_add(size_of::<MemoryMap>());
            unsafe {
                ptr.wrapping_add(index * usize::try_from(self.entry_size).unwrap()).to_ref()
            }
        } else {
            panic!("trying to get not existing memory");
        }
    }
}


pub struct MemoryEntries<'m> {
    index: usize,
    bytes_read: usize,
    memory_map: &'m MemoryMap
}

impl<'m> Iterator for MemoryEntries<'m> {
    type Item = &'m Memory;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.memory_map.len() {
            let memory = &self.memory_map[self.index];
            self.bytes_read += usize::try_from(self.memory_map.entry_size).unwrap();
            self.index += 1;  
            Some(memory)
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

