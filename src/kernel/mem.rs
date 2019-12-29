pub mod paging;
pub mod addr;
pub mod alloc;

pub unsafe fn setup_memory() {
    paging::init();
    alloc::init();
}

