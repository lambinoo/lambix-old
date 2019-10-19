pub mod page_alloc;
pub mod addr;

pub fn setup_memory() {
    page_alloc::init_page_alloc();
}

