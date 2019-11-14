pub type Result<T> = core::result::Result<T, AllocationError>;

pub trait Allocator : Sync {
    fn total_memory() -> usize;
    fn allocate(layout: Layout) -> Result<VirtAddr>;
    fn allocate_zeroed(layout: Layout) -> Result<VirtAddr> {
        let allocated_memory = Self::allocate(layout)?;
        unsafe if let Ok(ref p) = allocated_memory {
            write_bytes(allocated_memory.to_ptr(), 0, layout.size());
        }
        allocated_memory
    }
}


#[derive(Debug)]
pub enum AllocationError {
    OutOfMemory,
    OutOfVirtualMemory
}

pub fn init() {
    map::init();
    unsafe {
        DefaultPageAllocator::init()
            .expect("failed to add memory to the pool");

        early_kprintln!("memory available: {}", DefaultPageAllocator::total_memory());
    }
}

