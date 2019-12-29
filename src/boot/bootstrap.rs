use lib::*;
use crate::kernel;

#[no_mangle]
pub extern "C" fn kernel_bootstrap() -> ! {
    unsafe {
        disable_interrupts!();

        kernel::mem::setup_memory();
        kernel::idt::setup_idt();
    }

    loop {}
}


