mod entry;
mod table;
mod vector;
mod handler;

use entry::*;
use table::*;

pub use table::IDT;
pub use handler::*;
pub use entry::Entry;
pub use vector::*;

use core::mem::size_of;
use core::convert::TryFrom;

#[repr(packed)]
struct IDTRegister {
    size: u16,
    addr: *const IDT
}

pub unsafe fn set_for_current_cpu(idt: &'static IDT) {
    let register = IDTRegister {
        size: u16::try_from(size_of::<IDT>()).unwrap(),
        addr: idt as *const IDT
    };

    asm!("lidt $0" :: "r"(register) :: "volatile");
}

