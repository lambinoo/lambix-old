use crate::kernel::table::idt::*;

use lib::sync::*;
use lib::*;

use alloc::vec::Vec;

static LOADED_IDT: Vec<IDT> = Vec::new();

isr!{
    fn default_handler() {
        early_kprintln!("blank handler");
    }
}


pub fn setup_idt() {
    
}

