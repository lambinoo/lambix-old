use core::convert::TryFrom;
use core::ffi::{VaList, VaListImpl};
use core::mem::size_of;
use core::panic;

use core::fmt;

use ::alloc::vec;
use ::alloc::vec::Vec;

use lib::ffi::cstr::CStr;

use acpica::*;
use core::ffi::*;

use printf_compat::{format, output};

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
extern "C" fn AcpiOsAllocate(size: ACPI_SIZE) -> *mut c_void {
    let req_size = usize::try_from(size).unwrap();

    let buffer_size = if req_size % size_of::<u128>() != 0 {
        req_size / size_of::<u128>() + 1
    } else {
        req_size / size_of::<u128>()
    };

    let buffer = vec![0u128; buffer_size];
    early_kprintln!(
        "AcpiOsAllocate: {:X?} ({}/{} bytes, requested={})",
        buffer.as_ptr(),
        size,
        buffer.len() * size_of::<u128>(),
        size
    );
    Vec::leak(buffer).as_ptr() as _
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
    let mut rsdp_pointer: ACPI_PHYSICAL_ADDRESS = 0;
    unsafe { AcpiFindRootPointer(&mut rsdp_pointer) };
    early_kprintln!("RSDP Address {:?}", rsdp_pointer as *const u64);
    rsdp_pointer
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
        Err(_err) => core::ptr::null_mut(),
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

unsafe fn _handle_format(citer: &mut core::str::Chars, args: &mut VaList, fmt: &str) {
    while let Some(c) = citer.next() {
        match c {
            '%' => early_kprint!("%"),
            '0'..='9' | '.' | '-' => continue,
            'X' => early_kprint!("{:X}", args.arg::<c_uint>()),
            'x' => early_kprint!("{:x}", args.arg::<c_uint>()),
            'd' | 'i' => early_kprint!("{}", args.arg::<c_int>()),
            's' => {
                let ptr = args.arg::<*const c_char>();
                early_kprint!("[{:?}]", ptr);

                let str = CStr::from_ptr(ptr);
                early_kprint!("{}", str.as_str().unwrap())
            }
            _ => unimplemented!("Printf ACPI: {} is not handled (full format: {})", c, fmt),
        }
        break;
    }
}

unsafe fn _vprintf_impl(format: *const c_char, mut args: VaList) {
    let cstr = CStr::from_ptr(format).as_str().unwrap_or("not valid utf8");
    struct Data;
    impl fmt::Write for Data {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for c in s.chars() {
                early_kprint!("{}", c);
            }
            Ok(())
        }
    }

    format(format, args, output::fmt_write(&mut Data));
}

#[no_mangle]
unsafe extern "C" fn AcpiOsPrintf(format: *const c_char, args: ...) -> () {
    let mut a = args.clone();
    _vprintf_impl(format, a.as_va_list());
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
unsafe extern "C" fn AcpiOsVprintf(format: *const c_char, args: core::ffi::VaList) {
    let mut a = args.clone();
    _vprintf_impl(format, a.as_va_list());
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
