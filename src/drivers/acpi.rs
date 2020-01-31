mod acpica_os_layer;
mod acpica_dbg;
use acpica::*;

pub fn setup_acpi() {
    unsafe {
        AcpiInitializeTables(core::ptr::null_mut(), 0, 0);
    };
}

