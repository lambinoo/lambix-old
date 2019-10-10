use crate::kernel::table::idt::{Flags, InterruptDescriptorTable as IDT};
use lib::sync::*;
use lib::*;

pub static GLOBAL_IDT: Mutex<IDT> = Mutex::new(IDT::new());

isr!{
    fn divide_zero_handler() {
        early_kprint!("divide by zero handler");
        early_kprintln!();
    }

    fn blank_handler() {
        early_kprint!("a");
    }

    fn handler_80() {
        early_kprintln!("HANDLER 80 WOOOO");
    }
}


pub fn setup_idt() {
    let mut idt = GLOBAL_IDT.lock();

    for i in 0..=255 {
        idt.set_handler(i, Flags::PRESENT | Flags::TYPE_INTERRUPT, blank_handler);
    }

    idt.set_handler(0, Flags::PRESENT | Flags::TYPE_INTERRUPT, divide_zero_handler);
    idt.set_handler(1, Flags::PRESENT | Flags::TYPE_TRAP, blank_handler);
    idt.set_handler(3, Flags::PRESENT | Flags::TYPE_TRAP, blank_handler);
    idt.set_handler(4, Flags::PRESENT | Flags::TYPE_TRAP, blank_handler);

    idt.set_handler(80, Flags::PRESENT | Flags::TYPE_INTERRUPT, handler_80);

    unsafe {
        idt.load();
        enable_interrupts!();

        asm!("int $0" :: "N"(80) :: "volatile");
        asm!("int $0" :: "N"(0) :: "volatile");
    };
}

