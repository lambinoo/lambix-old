pub mod alloc;
pub mod paging;
pub mod addr;
pub mod vbox;
pub mod vbuffer;
mod valloc;

/// Reexport
pub use vbox::*;
pub use vbuffer::*;
pub use addr::*;
pub use paging::Flags;

use paging::*;
use crate::drivers::vga_buffer::*;
use crate::boot::multiboot;

use core::ptr::NonNull;

pub unsafe fn setup_memory() { 
    paging::init();
    alloc::init();
    valloc::init();
    multiboot::init(); 

    let vga_buffer_addr: *mut CharacterBuffer = VBox::into_raw(VBox::new(BASE_ADDR).unwrap());
    VGA_BUFFER.lock().set_buffer_addr(NonNull::new(vga_buffer_addr).unwrap());
    unmap2m(VirtAddr::from(0)).expect("failed to unmap, aborting");
}

