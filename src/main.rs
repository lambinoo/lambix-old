#![no_std]
#![no_main]
#![feature(
    const_fn,
    asm,
    format_args_nl,
    abi_x86_interrupt,
    panic_info_message,
    const_transmute,
    range_is_empty
)]

#[macro_use]
pub mod boot;
pub mod kernel;
pub mod panic;

use kernel::mem::page_alloc::*;
use core::sync::atomic::*;
use lib::isr;

static TIMER_COUNTER: AtomicU64 = AtomicU64::new(0);
 
isr! {
    fn apic_timer_handler() {
        TIMER_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    kernel::mem::setup_memory();
    kernel::idt::setup_idt();

    kernel::apic::setup_apic().expect("failed to setup apic");
    debug_assert!(kernel::apic::APIC.lock().is_available());

    early_kprintln!("{:?}", &kernel_main as *const _);

    loop {
        unsafe { asm!("hlt" :::: "volatile" ) };
    }
}



fn apic_test() {
    let mut apic = kernel::apic::APIC.lock();
    unsafe {
        use kernel::table::idt::Flags;
        kernel::idt::GLOBAL_IDT.lock().set_handler(
            200,
            Flags::PRESENT | Flags::TYPE_INTERRUPT,
            apic_timer_handler
        );

        apic.set_local_timer(0b110000000000000000 | 200).unwrap();
        apic.set_spurious_interrupt(0b1100000000 | 201).unwrap();
    };
}

