use super::Tag;

pub struct ACPIRsdp<T> {
    tag: Tag,
    rsdp: T
}

impl<T> ACPIRsdp<T> {
    pub fn get_rsdp(&self) -> &T {
        &self.rsdp 
    }

    pub fn get_rsdp_ptr(&self) -> *const T {
        self.get_rsdp() as _
    }
}

#[repr(packed)]
pub struct RsdpV1 {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32
}

#[repr(packed)]
pub struct RsdpV2 {
    pub rsdpv1: RsdpV1,
    pub length: u32,
    pub xsdt_addr: u64,
    pub extended_checksum: u8,
    reserved: [u8; 3]
}

