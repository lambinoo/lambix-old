pub mod paging;
pub mod addr;
pub mod alloc;

pub fn setup_memory() {
    paging::init();
    alloc::init();
}

