use ::lib::*;
use ::alloc::boxed::Box; 
use crate::kernel::config::*;
use crate::kernel::mem::addr::*;
use crate::kernel::table::paging::*;
pub use crate::kernel::table::paging::Flags;

pub type Result<T> = core::result::Result<T, MapErr>;

#[derive(Copy, Clone, Debug)]
pub enum MapErr {
    AlreadyMapped,
    NotMapped,
    OutOfMemory
}

pub unsafe fn get_physical_page(addr: VirtAddr) -> Result<PhyAddr> {
    let mut result = Err(MapErr::NotMapped);

    if PageTable::get_entry(PageTableType::PML4T, addr).is_present()
        && PageTable::get_entry(PageTableType::PDPT, addr).is_present()
        && PageTable::get_entry(PageTableType::PDT, addr).is_present() {

        let entry = PageTable::get_entry(PageTableType::PT, addr);
        if entry.is_present() {
            let paddr_mask = ((1 << 52) - 1) & !((1 << 12) - 1);
            result = Ok(PhyAddr::from(entry.get_value() & paddr_mask));
        }
    }

    result
}

/// Map a physical address to a virtual address.
/// If the memory is already mapped, returns an AlreadyMapped error containing the physical address
/// currently mapped to that virtual address.
///
/// # Safety 
/// You have to disable scheduling & interrupts when using this function as it's going
/// to change the address space for the current CPU only.
///
/// You have to ensure that the physical address is 4k-aligned, and doesn't have it's higher
/// significant bit (the 64th one) set as it's used for NX pages.
pub unsafe fn map4k(vaddr: VirtAddr, paddr: PhyAddr, flags: Flags)-> Result<()> {
    let root_entry = PageTable::get_entry(PageTableType::PML4T, vaddr);
    allocate_if_not_exist(root_entry);

    let pdpt_entry = PageTable::get_entry(PageTableType::PDPT, vaddr);
    allocate_if_not_exist(pdpt_entry);

    let pdt_entry = PageTable::get_entry(PageTableType::PDT, vaddr);
    allocate_if_not_exist(pdt_entry);

    let pt_entry = PageTable::get_entry(PageTableType::PT, vaddr);
    if !pt_entry.is_present() {
        pt_entry.set(paddr, flags);
        Ok(())
    } else {
        Err(MapErr::AlreadyMapped)
    }
}

unsafe fn allocate_if_not_exist(entry: &Entry) {
    if !entry.is_present() {
        let page_table = VirtAddr::from(Box::into_raw(PageTable::new()) as usize);
        entry.set(
            get_physical_page(page_table).expect("we just allocated it, it has to be mapped"),
            PageTable::default_flags()
        );
    }
}


/// Setup the address space that allows access to all page translation tables for the current CPU.
unsafe fn setup_paging_table_address_space() {
    // at that point of the boot process, we have memory mapped the 1st GB of the address space
    // so for every in-kernel address, VirtAddr <==> PhyAddr
    let root_table = VirtAddr::from(get_cr3!()).to_ref::<PageTable>();
    let base_addr = VirtAddr::from(PAGE_MAP_BASE);

    let index = PageTable::get_index(PageTableType::PML4T, base_addr);
    let flags = Flags::PRESENT | Flags::READ_WRITE | Flags::NO_EXECUTE;
    root_table[index].set_value(get_cr3!() | flags.bits());
    
    set_cr3!(get_cr3!()); // flush everything!
}


/// Initiliaze the pagging subsystem
pub unsafe fn init() {
    setup_paging_table_address_space();
}

