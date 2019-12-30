#![no_std]
#![no_main]
#![feature(
    const_fn,
    asm,
    format_args_nl,
    abi_x86_interrupt,
    panic_info_message,
    const_transmute,
    range_is_empty,
    alloc_error_handler,
    new_uninit,
    alloc_layout_extra
)]

extern crate alloc;

#[macro_use]
pub mod boot;
pub mod kernel;
pub mod panic;

#[no_mangle]
pub extern "Rust" fn kernel_main() -> ! {
    early_kprintln!("kernel main reached");
    loop {}
}

