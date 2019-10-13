use crate::kernel::table::idt::{Flags, InterruptDescriptorTable as IDT};
use lib::sync::*;
use lib::*;

pub static GLOBAL_IDT: Mutex<IDT> = Mutex::new(IDT::new());

isr!{
    fn blank_handler() {}

    fn ac_exception() {
        early_kprintln!("ac exception");
    }
}


pub fn setup_idt() {
    let mut idt = GLOBAL_IDT.lock();

    for i in 0..=255 {
        idt.set_handler(i, Flags::PRESENT | Flags::TYPE_INTERRUPT, blank_handler);
    }

    //idt.set_handler(17, Flags::PRESENT | Flags::TYPE_INTERRUPT, ac_exception);
    //idt.set_handler(13, Flags::PRESENT | Flags::TYPE_INTERRUPT, ac_exception);
    
    unsafe {
        idt.load();
        enable_interrupts!();
    };
}

