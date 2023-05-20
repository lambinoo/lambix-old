pub mod memmap;

use crate::kernel::mem::addr::*;
use memmap::*;

use core::convert::TryFrom;
use core::fmt;
use core::mem::size_of;
use core::ops::Range;
use core::ptr::NonNull;

use alloc::{vec, vec::Vec};

static mut BOOT_INFO: Option<BootInfo> = None;

#[derive(Copy)]
pub struct BootInfo {
    header: NonNull<InfoHeader>,
}

impl BootInfo {
    pub const unsafe fn at(header: NonNull<InfoHeader>) -> BootInfo {
        BootInfo { header }
    }

    pub fn len(&self) -> usize {
        unsafe { usize::try_from(self.header.as_ref().total_size).unwrap() }
    }

    pub fn range(&self) -> Range<VirtAddr> {
        let start = VirtAddr::from(self.header);
        start..start.wrapping_add(self.len())
    }

    /// Get the tags from the boot information at this address
    pub fn tags(&self) -> Tags {
        Tags {
            current_tag: VirtAddr::from(self.header.as_ptr().wrapping_add(size_of::<InfoHeader>())),
            _phantom: core::marker::PhantomData,
        }
    }

    pub fn get_tag(&self, tag_type: TagType) -> Option<Tag> {
        self.tags().filter(|tag| tag.tag_type() == tag_type).next()
    }

    fn get_ptr(&self) -> *const u8 {
        self.header.as_ptr() as *const u8
    }
}

impl Clone for BootInfo {
    fn clone(&self) -> BootInfo {
        #[repr(align(16))]
        #[derive(Copy, Clone)]
        pub struct BootInfoByte(u8);

        let data_ptr = self.header.as_ptr() as *const BootInfoByte;
        let length = self.len();
        let slice = unsafe { core::slice::from_raw_parts(data_ptr, length) };

        let mut new_boot_info: Vec<BootInfoByte> = vec![BootInfoByte(0); slice.len()];
        new_boot_info.copy_from_slice(slice);

        let boot_info = Vec::leak(new_boot_info);
        let header = NonNull::new(boot_info.as_ptr() as *mut InfoHeader).unwrap();
        BootInfo { header }
    }
}

pub struct Tags<'t> {
    current_tag: VirtAddr,
    _phantom: core::marker::PhantomData<&'t TagHeader>,
}

impl<'t> Tags<'t> {
    #[inline]
    fn get_current_tag(&self) -> Tag<'t> {
        let tag_header_ptr = self.current_tag.as_ptr() as *const TagHeader;
        let header = unsafe { &*tag_header_ptr };

        let data_size = usize::try_from(header.size).unwrap() - size_of::<TagHeader>();
        let tag_data_ptr = tag_header_ptr.wrapping_add(1) as *const u8;

        let data = unsafe { core::slice::from_raw_parts(tag_data_ptr, data_size) };

        Tag { header, data }
    }

    fn next_tag(&mut self) {
        let size = usize::try_from(self.get_current_tag().size()).unwrap();
        let unaligned_ptr = self.current_tag.wrapping_add(size);
        self.current_tag = unaligned_ptr.align::<u64>();
    }
}

impl<'t> Iterator for Tags<'t> {
    type Item = Tag<'t>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_tag = self.get_current_tag();
        match current_tag.tag_type() {
            TagType::EndTag => None,
            _ => {
                self.next_tag();
                Some(current_tag)
            }
        }
    }
}

#[repr(align(8))]
pub struct InfoHeader {
    total_size: u32,
    _reserved: u32,
}

#[repr(C)]
struct TagHeader {
    tag_type: TagType,
    size: u32,
}

pub struct Tag<'t> {
    header: &'t TagHeader,
    data: &'t [u8],
}

impl<'t> Tag<'t> {
    pub fn tag_type(&self) -> TagType {
        self.header.tag_type
    }

    pub fn size(&self) -> usize {
        usize::try_from(self.header.size).unwrap()
    }

    pub fn data(&self) -> &'t [u8] {
        let data_size = self.size() - size_of::<TagHeader>();
        unsafe { core::slice::from_raw_parts(self.data.as_ptr(), data_size) }
    }

    pub fn as_memmap(&self) -> Option<&'t MemoryMap> {
        match self.tag_type() {
            TagType::MemMap => MemoryMap::from_bytes(self.data()),
            _ => None,
        }
    }
}

impl<'t> fmt::Debug for Tag<'t> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tag {{ type: {:?}, size: {} }}",
            self.tag_type(),
            self.size()
        )
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
    ImageBasePhyAddr = 21,
}

extern "C" {
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
            let boot_info = BootInfo::at(
                NonNull::new(get_info_header_addr().as_mut_ptr::<InfoHeader>()).unwrap(),
            );
            BOOT_INFO = Some(boot_info.clone());

            let page_size = LALLOC.page_size();
            let range = boot_info.range();
            let mut page = PhyAddr::from(boot_info.get_ptr());
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
