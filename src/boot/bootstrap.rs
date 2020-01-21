use crate::kernel;
use crate::kernel_main;

use ::lib::*;
use ::alloc::boxed::Box;

#[repr(align(4096))]
struct Page([u8; 12000]);

#[no_mangle]
pub unsafe extern "C" fn kernel_bootstrap() -> ! {
    disable_interrupts!();

    kernel::mem::setup_memory();
    kernel::idt::setup_idt();
    kernel::apic::setup_apic();

    exec_with_new_stack(kernel_main);
}

unsafe fn exec_with_new_stack(f: unsafe fn() -> !) -> ! {
    let stack_page: *mut Page = Box::into_raw(Box::new_zeroed().assume_init());
    asm!("movq $0, %rbp" :: "r"(stack_page));
    asm!("movq $0, %rsp" :: "r"(stack_page));
    enable_interrupts!();
    f();
}

