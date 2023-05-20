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
    new_uninit
)]

extern crate alloc;


#[macro_use]
pub mod boot;
pub mod kernel;
pub mod drivers;
pub mod panic;

#[no_mangle]
pub fn kernel_main() -> ! {
    early_kprintln!("kernel_main reached");
    loop {
        early_kprintln!("tick");
        unsafe { core::arch::asm!("hlt"); };
    }
}
