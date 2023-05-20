mod lacpica;
pub mod tables;

use acpica::*;

pub fn setup_acpi() {
    unsafe {
        AcpiInitializeTables(core::ptr::null_mut(), 0, 0);
    };
}
