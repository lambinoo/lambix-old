#![no_std]
#![no_main]
#![feature(const_fn, asm, naked_functions, format_args_nl, abi_x86_interrupt, panic_info_message, core_intrinsics)]

#[macro_use]
pub mod boot;
pub mod kernel;
pub mod panic;

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    kernel::idt::setup_idt();
    // boot::setup_paging();
    // boot::setup_apic();
    loop {}
}

