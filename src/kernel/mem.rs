pub mod addr;
pub mod alloc;
pub mod paging;
mod valloc;
pub mod vbox;
pub mod vbuffer;

pub use addr::*;
pub use paging::Flags;
/// Reexport
pub use vbox::*;
pub use vbuffer::*;

use crate::boot::multiboot;
use crate::drivers::vga_buffer::*;
use paging::*;

use core::ptr::NonNull;

pub unsafe fn setup_memory() {
    paging::init();
    alloc::init();
    valloc::init();
    multiboot::init();

    let vga_buffer_addr: *mut CharacterBuffer = VBox::into_raw(VBox::new(BASE_ADDR).unwrap());
    VGA_BUFFER
        .lock()
        .set_buffer_addr(NonNull::new(vga_buffer_addr).unwrap());
    unmap2m(VirtAddr::from(0)).expect("failed to unmap, aborting");
}
