use core::convert::TryFrom;
use core::ffi::VaList;
use core::mem::size_of;

use core::fmt;

use ::alloc::vec;
use ::alloc::vec::Vec;

use acpica::*;
use core::ffi::*;
use lib::{io_read_port, io_write_port};

use crate::kernel::mem::*;
use printf_compat::{format, output};

#[no_mangle]
extern "C" fn AcpiOsAcquireLock() -> ACPI_STATUS {
    0
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
extern "C" fn AcpiOsEnterSleep(
    _SleepState: UINT8,
    _RegaValue: UINT32,
    _RegbValue: UINT32,
) -> ACPI_STATUS {
    0
}

#[no_mangle]
extern "C" fn AcpiOsExecute() -> () {
    unimplemented!();
}

#[no_mangle]
extern "C" fn AcpiOsFree(_ptr: *mut c_void) -> ACPI_STATUS {
    0 // TODO: proper allocator is needed..
}

#[no_mangle]
extern "C" fn AcpiOsGetRootPointer() -> ACPI_PHYSICAL_ADDRESS {
    let mut rsdp_pointer: ACPI_PHYSICAL_ADDRESS = 0;
    unsafe { AcpiFindRootPointer(&mut rsdp_pointer) };
    rsdp_pointer
}

#[no_mangle]
extern "C" fn AcpiOsGetThreadId() -> u64 {
    0
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
extern "C" fn AcpiOsPhysicalTableOverride(
    _ExistingTable: *mut ACPI_TABLE_HEADER,
    NewAddress: *mut ACPI_PHYSICAL_ADDRESS,
    NewTableLength: *mut UINT32,
) -> () {
    unsafe {
        *NewAddress = 0;
        *NewTableLength = 0;
    }
}

#[no_mangle]
extern "C" fn AcpiOsPredefinedOverride(
    _PredefinedObject: *mut ACPI_PREDEFINED_NAMES,
    NewValue: *mut ACPI_STRING,
) -> () {
    unsafe {
        *NewValue = core::ptr::null_mut() as ACPI_STRING;
    };
}

unsafe fn _vprintf_impl(fmt: *const c_char, args: VaList) {
    struct Data;
    impl fmt::Write for Data {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for c in s.chars() {
                early_kprint!("{}", c);
            }
            Ok(())
        }
    }

    format(fmt, args, output::fmt_write(&mut Data));
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
extern "C" fn AcpiOsReadPort(
    Address: ACPI_IO_ADDRESS,
    Value: *mut UINT32,
    Width: UINT32,
) -> ACPI_STATUS {
    unsafe {
        *Value = 0;
        *Value = match Width {
            8 => io_read_port!(u8, Address) as u32,
            16 => io_read_port!(u16, Address) as u32,
            32 => io_read_port!(u32, Address) as u32,
            _ => {
                return 1;
            }
        };

        early_kprintln!("Read port: 0x{:x} {} => {:?}", Address, Width, *Value);
    }

    0
}

#[no_mangle]
extern "C" fn AcpiOsReleaseLock() -> ACPI_STATUS {
    0
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
extern "C" fn AcpiOsSignalSemaphore(_Handle: *const (), _Units: UINT32) -> ACPI_STATUS {
    1
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
extern "C" fn AcpiOsTableOverride(
    ExistingTable: *const ACPI_TABLE_HEADER,
    NewTable: *mut *const ACPI_TABLE_HEADER,
) -> () {
    unsafe {
        *NewTable = ExistingTable;
    };
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
extern "C" fn AcpiOsWaitSemaphore(
    _Handle: *mut c_void,
    _Units: UINT32,
    _Timeout: UINT16,
) -> ACPI_STATUS {
    0
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
extern "C" fn AcpiOsWritePort(
    Address: ACPI_IO_ADDRESS,
    Value: UINT32,
    Width: UINT32,
) -> ACPI_STATUS {
    unsafe {
        early_kprintln!("Write port: 0x{:x} {} <= {:?}", Address, Width, Value);
        match Width {
            8 => io_write_port!(u8, Address, Value),
            16 => io_write_port!(u16, Address, Value),
            32 => io_write_port!(u8, Address, Value),
            _ => {
                return 1;
            }
        }
    }

    0
}
