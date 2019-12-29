pub mod memmap;

use crate::kernel::mem::addr::*;
use core::convert::TryFrom;
use core::ops::Range;
use core::mem::size_of;
use memmap::*;

pub static BOOT_INFO: BootInfo = BootInfo;

extern {
    static multiboot_header_addr: u32;
}

pub struct BootInfo;

impl BootInfo {
    pub fn paddr(&self) -> PhyAddr {
        unsafe { PhyAddr::from(usize::try_from(multiboot_header_addr).unwrap()) }
    }

    pub fn range(&self) -> Range<PhyAddr> {
        unsafe {
            let total_size = usize::try_from(
                self.paddr().to_ref::<InfoHeader>().total_size
            ).unwrap();

            self.paddr() .. self.paddr().wrapping_add(total_size)
        }
    }

    /// Get the tags from the boot information at this address
    pub unsafe fn tags(&self, vaddr: VirtAddr) -> Tags {
        Tags {
            current_tag: vaddr.wrapping_add(size_of::<InfoHeader>()),
            _phantom: core::marker::PhantomData
        }
    }
}

#[derive(Copy, Clone)]
pub struct Tags<'t> {
    current_tag: VirtAddr,
    _phantom: core::marker::PhantomData<&'t Tag>
}

impl<'t> Iterator for Tags<'t> {
    type Item = &'t Tag;

    fn next(&mut self) -> Option<Self::Item> {
        let current_tag = self.get_current_tag();
        match current_tag.tag_type {
            TagType::EndTag => None,
            _  => {
                self.next_tag();
                Some(current_tag)
            }
        }
    }
}

impl<'t> Tags<'t> {
    #[inline]
    fn get_current_tag(&self) -> &'t Tag {
        unsafe { self.current_tag.to_ref() }
    }

    pub fn next_tag(&mut self) {
        let size = usize::try_from(self.get_current_tag().size).unwrap();
        let unaligned_ptr = self.current_tag.wrapping_add(size); 
        self.current_tag = unaligned_ptr.align::<u64>();
    }
}


#[repr(C)]
#[derive(Debug)]
pub struct InfoHeader {
    total_size: u32,
    _reserved: u32
}

#[derive(Debug)]
pub struct Tag {
    pub tag_type: TagType,
    pub size: u32
}

impl Tag {
    fn payload<T>(&self) -> &T {
        unsafe { VirtAddr::from(self).to_ref() }
    }

    pub fn as_memmap(&self) -> Option<&MemoryMap> {
        match self.tag_type {
            TagType::MemMap => Some(self.payload()),
            _ => None
        }
    }
}


#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TagType {
    EndTag = 0,
    BasicMemInfo = 4,
    BIOSBootDevice = 5,
    CmdLine = 1,
    Modules = 3,
    ELFSymbols = 9,
    MemMap = 6,
    BootLoaderName = 2,
    ApmTable = 10,
    VbeInfo = 7,
    FramebufferInfo = 8,
    EFI32TablePointer = 11,
    EFI64TablePointer = 12,
    SmbiosTables = 13,
    ACPIOldRsdp = 14,
    ACPINewRsdp = 15,
    NetInfo = 16,
    EFIMemMap = 17,
    EFIBootServicesNotTerminated = 18,
    EFI32ImagePointer = 19,
    EFI64ImagePointer = 20,
    ImageBasePhyAddr = 21
}

