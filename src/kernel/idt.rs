use crate::kernel::table::idt::{Flags, InterruptDescriptorTable as IDT};
use lib::sync::*;
use lib::*;

pub static GLOBAL_IDT: Spinlock<IDT> = Spinlock::new(IDT::new());

macro_rules! print_interrupt {
    ($idt:ident, $name:ident, $vector:expr) => {
        isr! {
            fn $name() {
                early_kprintln!("int {} ({})", stringify!($name), $vector);
            }
        }
        
        $idt.set_handler($vector, Flags::PRESENT | Flags::TYPE_INTERRUPT, $name);
    }
}

isr!{
    fn blank_handler() {
        early_kprintln!("blank handler");
    }
}


pub fn setup_idt() {
    let mut idt = GLOBAL_IDT.lock();

    for i in 0..=255 {
        idt.set_handler(i, Flags::PRESENT | Flags::TYPE_INTERRUPT, blank_handler);
    }

    print_interrupt!(idt, int0, 0);
    print_interrupt!(idt, int1, 1);
    print_interrupt!(idt, int2, 2);
    print_interrupt!(idt, int3, 3);
    print_interrupt!(idt, int4, 4);
    print_interrupt!(idt, int5, 5);
    print_interrupt!(idt, int6, 6);
    print_interrupt!(idt, int7, 7);
    print_interrupt!(idt, int8, 8);
    print_interrupt!(idt, int9, 9);
    print_interrupt!(idt, int10, 10);
    print_interrupt!(idt, int11, 11);
    print_interrupt!(idt, int12, 12);
    print_interrupt!(idt, int13, 13);
    print_interrupt!(idt, int14, 14);
    print_interrupt!(idt, int15, 15);
    print_interrupt!(idt, int16, 16);
    print_interrupt!(idt, int17, 17);
    print_interrupt!(idt, int18, 18);
    print_interrupt!(idt, int19, 19);
    print_interrupt!(idt, int20, 20);
    print_interrupt!(idt, int21, 21);
    print_interrupt!(idt, int22, 22);
    print_interrupt!(idt, int23, 23);
    print_interrupt!(idt, int24, 24);
    print_interrupt!(idt, int25, 25);
    print_interrupt!(idt, int26, 26);
    print_interrupt!(idt, int27, 27);
    print_interrupt!(idt, int28, 28);
    print_interrupt!(idt, int29, 29);
    print_interrupt!(idt, int30, 30);
    print_interrupt!(idt, int31, 31);

    early_kprintln!("{:#?}", &int8 as *const _);
    
    unsafe {
        idt.load();
        enable_interrupts!();
    };
}

