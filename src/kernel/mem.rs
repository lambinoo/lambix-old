pub mod paging;
pub mod addr;
pub mod alloc;
pub mod vbox;

pub unsafe fn setup_memory() {
    paging::init();
    alloc::init();
    vbox::init();
}

