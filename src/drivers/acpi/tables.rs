#[repr(packed)]
struct RsdpV1Header {
    pub signature: [u8; 8],
    pub checksum: u8,
    pub oemid: [u8; 6],
    pub revision: u8,
    pub rsdt_addr: u32,
}

#[repr(packed)]
struct RsdpV2Header {
    pub rsdpv1: RsdpV1Header,
    pub length: u32,
    pub xsdt_addr: u64,
    pub extended_checksum: u8,
    reserved: [u8; 3],
}

#[repr(packed)]
pub struct RsdpV1 {
    header: RsdpV1Header,
    tables_addr: [u32],
}

#[repr(packed)]
pub struct RsdpV2 {
    header: RsdpV2Header,
    tables_addr: [u64],
}
