pub mod memmap;

use crate::kernel::mem::addr::*;
use memmap::*;

use core::convert::TryFrom;
use core::ops::Range;
use core::mem::size_of;
use core::ptr::NonNull;

static mut BOOT_INFO: Option<BootInfo> = None;

#[derive(Copy)]
pub struct BootInfo {
    header: NonNull<InfoHeader>
}

impl BootInfo { 
    pub const unsafe fn at(header: NonNull<InfoHeader>) -> BootInfo {
        BootInfo { header }
    }
 
    pub fn len(&self) -> usize {
        early_kprintln!("bloop");
        unsafe {
            usize::try_from(self.header.as_ref().total_size).unwrap()
        }
    } 

    pub fn range(&self) -> Range<VirtAddr> {
        let start = VirtAddr::from(self.header);
        start .. start.wrapping_add(self.len())
    }

    /// Get the tags from the boot information at this address
    pub fn tags(&self) -> Tags {
        Tags {
            current_tag: VirtAddr::from(
                self.header.as_ptr().wrapping_add(size_of::<InfoHeader>())
            ),
            _phantom: core::marker::PhantomData
        }
    }

    fn get_ptr(&self) -> *const u8 {
        self.header.as_ptr() as *const u8
    }
}

impl Clone for BootInfo {
    fn clone(&self) -> BootInfo {
        use ::alloc::{boxed::Box, *};

        let total_size = self.len();

        let vector_capacity = total_size / size_of::<InfoHeader>()
            + if total_size % size_of::<BootInfo>() != 0 { 1 } else { 0 };

        let owned_boot_info = vec![0u64; vector_capacity];
        let boot_info = self.get_ptr();

        unsafe {
            core::ptr::copy(boot_info, owned_boot_info.as_ptr() as *mut u8, total_size);
        };

        let owned_boot_info_ptr = Box::into_raw(owned_boot_info.into_boxed_slice());
    
        unsafe {
            BootInfo::at(NonNull::new(owned_boot_info_ptr as *mut InfoHeader).unwrap())
        }
    }
}


pub struct Tags<'t> {
    current_tag: VirtAddr,
    _phantom: core::marker::PhantomData<&'t Tag>
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


#[repr(align(8))]
pub struct InfoHeader {
    total_size: u32,
    _reserved: u32
}

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

extern {
    static multiboot_header_addr: u32;
}

pub fn get_info_header_addr() -> PhyAddr {
    PhyAddr::new(usize::try_from(unsafe { multiboot_header_addr }).unwrap())
}

pub fn get_boot_info() -> BootInfo {
    unsafe { BOOT_INFO.expect("boot information hasn't been copied yet") }
}

pub fn init() {
    use crate::kernel::mem::alloc::LALLOC;

    unsafe {
        if BOOT_INFO.is_none() {
            let boot_info = BootInfo::at(NonNull::new(get_info_header_addr().as_mut_ptr::<InfoHeader>()).unwrap());
            BOOT_INFO = Some(boot_info.clone());
            
            let page_size = LALLOC.page_size();
            let range = BOOT_INFO.unwrap().range();
            let mut page = PhyAddr::from(BOOT_INFO.unwrap().get_ptr()) &! (page_size - 1);
            while usize::from(page) < usize::from(range.end) {
                LALLOC.add_page_to_memory_pool(page);
                page = page.wrapping_add(page_size);
            }
        } else {
            panic!("boot information were already initialized");
        }
    }

    early_kprintln!("boot info initialized");
}

