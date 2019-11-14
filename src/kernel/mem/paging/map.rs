use lib::*;
use super::{PageAllocator, MemoryPage};
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


pub unsafe fn virt_to_phy_addr(vaddr: VirtAddr) -> Result<PhyAddr> {
    let entry = get_map_entry(vaddr)?;
    Ok(entry.base_addr() | usize::from(vaddr & 0xfff))
}


pub unsafe fn get_map_entry<'a>(vaddr: VirtAddr) -> Result<&'a Entry<PageTable>> {
    let mut entry = Err(MapErr::NotMapped);

    let base_off = PageMapLevel4Table::offset_of(VirtAddr::from(PAGE_MAP_BASE));
   
    let pml4t_off = PageMapLevel4Table::offset_of(vaddr);
    let pdpt_off  = PageDirectoryPointerTable::offset_of(vaddr);
    let pdt_off = PageDirectoryTable::offset_of(vaddr);
    let pt_off = PageTable::offset_of(vaddr);

    let pml4t: &PageMapLevel4Table = get_table_addr(base_off, base_off, base_off).to_ref();
    let pdpt: &PageDirectoryPointerTable = get_table_addr(base_off, base_off, pml4t_off).to_ref();
    let pdt: &PageDirectoryTable = get_table_addr(base_off, pml4t_off, pdpt_off).to_ref();
    let pt: &PageTable = get_table_addr(pml4t_off, pdpt_off, pdt_off).to_ref(); 

    if pml4t[pml4t_off].is_present() {
        if pdpt[pdpt_off].is_present() {
            if pdt[pdt_off].is_present() {
                if pt[pt_off].is_present() {
                    entry = Ok(&pt[pt_off]);
                }
            }
        }
    }

    entry
}


pub unsafe fn map4k<A: PageAllocator>(paddr: PhyAddr, vaddr: VirtAddr, flags: Flags) -> Result<()> {
    debug_assert!(vaddr.is_aligned(A::Page::PAGE_SIZE));
    debug_assert!(paddr.is_aligned(A::Page::PAGE_SIZE));

    // TODO disable preempt when mapping a new page (interrupt & scheduling)

    let base_off = PageMapLevel4Table::offset_of(VirtAddr::from(PAGE_MAP_BASE));
    let pml4t_off = PageMapLevel4Table::offset_of(vaddr);
    let pdpt_off  = PageDirectoryPointerTable::offset_of(vaddr);
    let pdt_off = PageDirectoryTable::offset_of(vaddr);
    let pt_off = PageTable::offset_of(vaddr);

    let pml4t: &PageMapLevel4Table = get_table_addr(base_off, base_off, base_off).to_ref();
    let pdpt: &PageDirectoryPointerTable = get_table_addr(base_off, base_off, pml4t_off).to_ref();
    let pdt: &PageDirectoryTable = get_table_addr(base_off, pml4t_off, pdpt_off).to_ref();
    let pt: &PageTable = get_table_addr(pml4t_off, pdpt_off, pdt_off).to_ref();

    alloc_page_if_not_exist::<A, PageMapLevel4Table>(&pml4t[pml4t_off])?;
    alloc_page_if_not_exist::<A, PageDirectoryPointerTable>(&pdpt[pdpt_off])?;
    alloc_page_if_not_exist::<A, PageDirectoryTable>(&pdt[pdt_off])?;

    if !pt[pt_off].is_present() {
        let flags = (Flags::PRESENT | flags).bits();
        pt[vaddr].set_value(flags | usize::from(paddr));
        Ok(())
    } else {
        Err(MapErr::AlreadyMapped(vaddr, paddr))
    }
}

pub unsafe fn unmap4k<A: PageAllocator>(vaddr: VirtAddr) -> Result<PhyAddr> {
    // TODO disable preempt when mapping a new page (interrupt & scheduling)
    
    let base_off = PageMapLevel4Table::offset_of(VirtAddr::from(PAGE_MAP_BASE));
   
    let pml4t_off = PageMapLevel4Table::offset_of(vaddr);
    let pdpt_off  = PageDirectoryPointerTable::offset_of(vaddr);
    let pdt_off = PageDirectoryTable::offset_of(vaddr);
    let pt_off = PageTable::offset_of(vaddr);

    let pml4t: &PageMapLevel4Table = get_table_addr(base_off, base_off, base_off).to_ref();
    let pdpt: &PageDirectoryPointerTable = get_table_addr(base_off, base_off, pml4t_off).to_ref();
    let pdt: &PageDirectoryTable = get_table_addr(base_off, pml4t_off, pdpt_off).to_ref();
    let pt: &PageTable = get_table_addr(pml4t_off, pdpt_off, pdt_off).to_ref(); 

    // TODO free empty page tables
    let mut base_addr = Err(MapErr::NotMapped);
    if pml4t[pml4t_off].is_present() {
        if pdpt[pdpt_off].is_present() {
            if pdt[pdt_off].is_present() {
                if pt[pt_off].is_present() {
                    let addr = pt[pt_off].base_addr();
                    pt[pt_off].set_value(0);
                    base_addr = Ok(addr);
                }
            }
        }
    }

    base_addr
}


pub fn init() {
    let paging_table_phy_addr = PhyAddr::from(get_cr3!());
    let paging_table = unsafe { paging_table_phy_addr.to_ref::<PageMapLevel4Table>() };
    let base_off = PageMapLevel4Table::offset_of(VirtAddr::from(PAGE_MAP_BASE));

    let flags = Flags::PRESENT | Flags::READ_WRITE | Flags::NO_EXECUTE;
    let entry = usize::from(paging_table_phy_addr) | flags.bits();
    paging_table[base_off].set_value(entry);
    
    unsafe {
        // flush
        set_cr3!(get_cr3!());
    };
}


fn alloc_page_if_not_exist<A, T>(entry: &Entry<T>) -> Result<()>
    where A: PageAllocator
{

    if !entry.is_present() {
        match A::allocate_zeroed() {
            Ok(page) => {
                let flags = (Flags::PRESENT | Flags::NO_EXECUTE | Flags::READ_WRITE).bits();
                entry.set_value(usize::from(page.paddr()) | flags);
                core::mem::forget(page);
                Ok(())
            }

            Err(_) => Err(MapErr::OutOfMemory)
        }
    } else {
        Ok(())
    }
}


fn get_table_addr(pdpt: usize, pdt: usize, pt: usize) -> VirtAddr {
    let addr = VirtAddr::from(PAGE_MAP_BASE | (pdpt << 30) | (pdt << 21) | (pt << 12));
    addr
}


