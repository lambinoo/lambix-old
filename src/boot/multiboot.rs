pub mod memmap;

use core::mem::size_of;
use memmap::*;

pub static BOOT_INFO: BootInfo = BootInfo;

extern {
    static multiboot_header_addr: u32;
}

pub struct BootInfo;

impl BootInfo {
    #[inline]
    fn ptr(&self) -> *const InfoHeader {
        unsafe { (multiboot_header_addr as u64) as _ }
    }

    #[inline]
    fn header(&self) -> &InfoHeader {
        unsafe { &*self.ptr() }
    }

    pub fn tags(&self) -> Tags {
        Tags {
            current_tag: self.ptr().wrapping_add(1) as _,
            _phantom: core::marker::PhantomData
        }
    }
}


pub struct Tags<'t> {
    current_tag: *const u8,
    _phantom: core::marker::PhantomData<&'t Tag>
}

impl<'t> Iterator for Tags<'t> {
    type Item = &'t Tag;

    fn next(&mut self) -> Option<Self::Item> {
        let current_tag = self.get_current_tag();
        match current_tag.tag_type {
            TagType::EndTag => {
                Some(self.get_current_tag())
            },
            _  => {
                self.next_tag();
                Some(self.get_current_tag())
            }
        }
    }
}

impl<'t> Tags<'t> {
    #[inline]
    fn get_current_tag(&self) -> &'t Tag {
        unsafe { &*(self.current_tag as *const Tag) }
    }

    pub fn next_tag(&mut self) {
        let size = self.get_current_tag().size as usize;
        let unaligned_ptr = self.current_tag.wrapping_add(size);
        self.current_tag = unaligned_ptr.wrapping_add(
            unaligned_ptr.align_offset(size_of::<Tag>())
        );
    }
}


#[repr(C)]
pub struct InfoHeader {
    max_size: u32,
    _reserved: u32
}

#[repr(C)]
#[derive(Debug)]
pub struct Tag {
    pub tag_type: TagType,
    pub size: u32
}

impl Tag {
    fn payload<T>(&self) -> &T {
        unsafe { &*(self as *const Tag as *const _) }
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

