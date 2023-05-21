#![no_std]
#![no_main]
#![feature(
    format_args_nl,
    abi_x86_interrupt,
    panic_info_message,
    alloc_error_handler,
    alloc_layout_extra,
    naked_functions,
    c_variadic,
    new_uninit,
    const_mut_refs
)]

use lib::disable_interrupts;

extern crate acpica;
extern crate alloc;

#[macro_use]
pub mod boot;
pub mod drivers;
pub mod kernel;
pub mod panic;

fn shutdown() {
    unsafe {
        acpica::AcpiEnterSleepStatePrep(5);
        disable_interrupts!();
        acpica::AcpiEnterSleepState(5);
    }
}

#[no_mangle]
pub fn kernel_main() -> ! {
    early_kprintln!("kernel_main reached");

    loop {
        early_kprintln!("tick");
        unsafe {
            core::arch::asm!("hlt");
        };
    }
}
