use crate::kernel::table::idt::*;

pub unsafe fn setup_idt() {
    let new_idt = IDT::new();
    IDT::set_for_this_cpu(new_idt);
}

