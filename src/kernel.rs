pub mod config;
pub mod table;
pub mod idt;
pub mod apic;
pub mod mem;

use core::ops::Range;
use mem::addr::PhyAddr;

extern {
    static kernel_start_addr: u8;
    static kernel_end_addr: u8; 
}

pub fn kernel_range() -> Range<PhyAddr> {
    unsafe {
        Range {
            start: PhyAddr::from(&kernel_start_addr),
            end: PhyAddr::from(&kernel_end_addr)
        }
    }
}

