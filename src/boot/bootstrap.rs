use crate::drivers;
use crate::kernel;
use crate::kernel_main;

use ::alloc::boxed::Box;
use ::lib::*;

#[repr(align(4096))]
struct Page([u8; 12000]);

#[no_mangle]
pub unsafe extern "C" fn kernel_bootstrap() -> ! {
    disable_interrupts!();

    kernel::mem::setup_memory();
    kernel::idt::setup_idt();
    kernel::apic::setup_apic();
    drivers::acpi::setup_acpi();

    exec_with_new_stack(kernel_main);
}

unsafe fn exec_with_new_stack(f: unsafe fn() -> !) -> ! {
    let stack_page: *mut Page = Box::into_raw(Box::new_zeroed().assume_init());
    core::arch::asm!("mov rbp, {stack}",
                     "mov rsp, {stack}",
                     stack = in(reg) stack_page);
    f();
}
