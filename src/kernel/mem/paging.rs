use lib::*;
use crate::kernel::config::*;
use crate::kernel::mem::addr::*;
use crate::kernel::table::paging::*;
pub use crate::kernel::table::paging::Flags;

pub type Result<T> = core::result::Result<T, MapErr>;

#[derive(Copy, Clone, Debug)]
pub enum MapErr {
    AlreadyMapped(VirtAddr, PhyAddr),
    NotMapped,
    OutOfMemory
}

pub fn init() {
    setup_paging_table_address_space();
}

fn setup_paging_table_address_space() {
    // at that point of the boot process, we have memory mapped the 1st GB of the address space
    // so for every in-kernel address, VirtAddr <==> PhyAddr
    let root_table = unsafe { VirtAddr::from(get_cr3!()).to_ref::<PageTable>() };
    let base_addr = VirtAddr::from(PAGE_MAP_BASE);

    let index = PageTable::get_index(PageTableType::PML4T, base_addr);

    let flags = Flags::PRESENT | Flags::READ_WRITE | Flags::NO_EXECUTE;
    let entry = get_cr3!() | flags.bits();

    root_table[index].set_value(get_cr3!() | flags.bits());
    
    unsafe {
        set_cr3!(get_cr3!()); // flush everything!
    };
}

