pub mod apic;
pub mod config;
pub mod idt;
pub mod mem;
pub mod table;

use core::ops::Range;
use mem::addr::PhyAddr;

extern "C" {
    static kernel_start_addr: usize;
    static kernel_end_addr: usize;

    static __eh_frame_hdr: usize;
    static __eh_frame: usize;
}

pub fn get_eh_frame_hdr_ptr() -> *const () {
    return unsafe { __eh_frame_hdr as *const () };
}

pub fn get_eh_frame_ptr() -> *const () {
    return unsafe { __eh_frame as *const () };
}

pub fn kernel_range() -> Range<PhyAddr> {
    unsafe {
        Range {
            start: PhyAddr::from(&kernel_start_addr),
            end: PhyAddr::from(&kernel_end_addr),
        }
    }
}
