use core::convert::TryFrom;
use core::mem::size_of;

use acpica::*;

use alloc::vec;
use alloc::vec::Vec;

use crate::boot::multiboot::{ get_boot_info, TagType };
use crate::kernel::mem::addr::*;
use crate::kernel::mem::paging::get_physical_address;

#[no_mangle]
extern "C" fn AcpiOsAcquireLock() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsAcquireObject() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsAllocate(size: ACPI_SIZE) -> *mut [u128] {
    early_kprintln!("allocating {} bytes for acpi", size);
    let req_size = usize::try_from(size).unwrap();

    let buffer_size = if req_size % size_of::<u128>() != 0 {
        req_size / size_of::<u128>() + 1
    } else {
        req_size / size_of::<u128>()
    };

#[no_mangle]
    let buffer = vec![0u128; buffer_size];
    Vec::leak(buffer) as _
}


#[no_mangle]
extern "C" fn AcpiOsCreateCache() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsCreateLock() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsCreateSemaphore() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsDeleteCache() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsDeleteLock() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsDeleteSemaphore() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsEnterSleep() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsExecute() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsFree() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsGetRootPointer() -> ACPI_PHYSICAL_ADDRESS {
    let boot_info = get_boot_info();

    let physical_addr = if let Some(rsdp) = boot_info.get_tag(TagType::ACPINewRsdp) {
        ACPI_PHYSICAL_ADDRESS::try_from(
            usize::from(unsafe {
                get_physical_address(VirtAddr::from(rsdp)).unwrap()
            })
        ).unwrap()
    } else {
        if let Some(rsdp) = boot_info.get_tag(TagType::ACPIOldRsdp) {
            ACPI_PHYSICAL_ADDRESS::try_from(
                usize::from(unsafe {
                    get_physical_address(VirtAddr::from(rsdp)).unwrap()
                })
            ).unwrap()
        } else {
            let mut acpi_phy_addr = 0;
            unsafe { AcpiFindRootPointer(&mut acpi_phy_addr as _) };
            acpi_phy_addr
        }
    };

    early_kprintln!("acpi get root pointer: 0x{:x}", physical_addr);

    physical_addr
}


#[no_mangle]
extern "C" fn AcpiOsGetThreadId() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsGetTimer() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsInitialize() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsInstallInterruptHandler() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsMapMemory(paddr: acpica::ACPI_PHYSICAL_ADDRESS, size: acpica::ACPI_SIZE) -> *mut u8 {
    early_kprintln!("os map memory paddr: 0x{:x} size: {}", paddr, size);
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsPhysicalTableOverride() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsPredefinedOverride() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsPrintf() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsPurgeCache() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsReadMemory() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsReadPciConfiguration() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsReadPort() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsReleaseLock() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsReleaseObject() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsRemoveInterruptHandler() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsSignal() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsSignalSemaphore() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsSleep() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsStall() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsTableOverride() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsTerminate() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsUnmapMemory() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsVprintf() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsWaitEventsComplete() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsWaitSemaphore() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsWriteMemory() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsWritePciConfiguration() -> () {
    unimplemented!();
}


#[no_mangle]
extern "C" fn AcpiOsWritePort() -> () {
    unimplemented!();
}

