use core::convert::TryFrom;
use core::mem::size_of;

use ::alloc::vec;
use ::alloc::vec::Vec;

use lib::ffi::cstr::CStr;

use acpica::*;
use cty::*;

use crate::boot::multiboot::{get_boot_info, TagType};
use crate::kernel::mem::paging::get_physical_address;
use crate::kernel::mem::*;

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
        let rsdp_addr = VirtAddr::from(rsdp.data().as_ptr());
        ACPI_PHYSICAL_ADDRESS::try_from(usize::from(unsafe {
            get_physical_address(rsdp_addr).unwrap()
        }))
        .unwrap()
    } else if let Some(rsdp) = boot_info.get_tag(TagType::ACPIOldRsdp) {
        let rsdp = VirtAddr::from(rsdp.data().as_ptr());
        ACPI_PHYSICAL_ADDRESS::try_from(usize::from(unsafe {
            get_physical_address(VirtAddr::from(rsdp)).unwrap()
        }))
        .unwrap()
    } else {
        let mut acpi_phy_addr = 0;
        unsafe { AcpiFindRootPointer(&mut acpi_phy_addr as _) };
        acpi_phy_addr
    };

    physical_addr
}

#[no_mangle]
extern "C" fn AcpiOsGetThreadId() -> u64 {
    unimplemented!()
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
extern "C" fn AcpiOsMapMemory(
    paddr: acpica::ACPI_PHYSICAL_ADDRESS,
    size: acpica::ACPI_SIZE,
) -> *mut u8 {
    let vbuffer = unsafe {
        VBuffer::with_flags(
            PhyAddr::from(usize::try_from(paddr).unwrap()),
            usize::try_from(size).unwrap(),
            Flags::NO_EXECUTE,
        )
    };

    match vbuffer {
        Ok(vbuffer) => VBuffer::leak(vbuffer).0,
        Err(err) => core::ptr::null_mut(),
    }
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
unsafe extern "C" fn AcpiOsPrintf(format: *const c_char, mut _args: ...) -> () {
    let cstr = CStr::from_ptr(format).as_str().unwrap_or("not valid utf8");
    early_kprint!("{}", cstr);
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
extern "C" fn AcpiOsUnmapMemory(addr: *const c_void, size: ACPI_SIZE) {
    let size = usize::try_from(size).unwrap();
    core::mem::drop(unsafe { VBuffer::from_raw(addr as _, size) });
}

#[no_mangle]
extern "C" fn AcpiOsVprintf(fmt: *const c_char, mut _va_list: core::ffi::VaList) {
    early_kprint!("{}", unsafe { CStr::from_ptr(fmt).as_str().unwrap() });
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
