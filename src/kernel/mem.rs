pub mod paging;
pub mod addr;
pub mod alloc;
pub mod vbox;

pub unsafe fn setup_memory() {
    use addr::*;
    use paging::*;
    use vbox::*;
    use crate::boot::vga_buffer::*;
    use core::ptr::NonNull;
    use crate::boot::multiboot;

    paging::init();
    alloc::init();
    vbox::init();
    multiboot::init(); 

    let vga_buffer_addr: *mut CharacterBuffer = VBox::into_raw(VBox::new(BASE_ADDR));
    VGA_BUFFER.lock().set_buffer_addr(NonNull::new(vga_buffer_addr).unwrap());
    unmap2m(VirtAddr::from(0)).expect("failed to unmap, aborting");
}

