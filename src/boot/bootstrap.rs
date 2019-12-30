use lib::*;
use crate::kernel;
use crate::kernel_main;
use crate::kernel::mem::addr::*;
use crate::kernel::mem::vbox::VBox;

use core::convert::TryFrom;
use alloc::string::*;

#[no_mangle]
pub unsafe extern "C" fn kernel_bootstrap() -> ! {
    disable_interrupts!();

    kernel::mem::setup_memory();
    kernel::idt::setup_idt();

    setup_apic();

    kernel_main();
}

#[repr(align(4096))]
struct APICRegisters {
    registers: [u16; 256]
}

impl core::fmt::Debug for APICRegisters {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut content = String::new();

        for i in 0..256 {
            let reg = self.registers[i];
            let value = unsafe { core::ptr::read_volatile(reg as *const u16) };
            content += &value.to_string();
            content += ", ";
        }

        write!(f, "APICRegisters {{ {} }}", content)
    }
}

unsafe fn setup_apic() {
    let apic_msr_reg = 0x000_001b;
    let mut apic_addr_reg = readmsr!(apic_msr_reg);
    apic_addr_reg[1] |= 1 << 11;
    writemsr!(apic_msr_reg, apic_addr_reg);

    let mask: u64 = (1 << 12) - 1;
    let apic_base_addr = PhyAddr::from(usize::try_from(
        (u64::from(apic_addr_reg[1]) | u64::from(apic_addr_reg[0]) << 32)
        &! (mask | (mask << 52))
    ).unwrap());

    let apic: VBox<APICRegisters> = VBox::new(apic_base_addr);
    early_kprintln!("{:?}", apic);
    loop {}
}


/*
unsafe fn remap_kernel_and_jump() -> ! {
    use crate::kernel::config::*;
    use crate::kernel::mem::paging::*;
    use crate::kernel::mem::addr::*;

    let kernel_range = kernel::kernel_range();
    let high_range = VirtAddr::from(KERNEL_START) .. VirtAddr::from(KERNEL_END);
    
    let max_size = high_range.start.distance(high_range.end);
    let size = kernel_range.start.distance(kernel_range.end);

    if size < max_size {
        for offset in (0..size).step_by(PAGE_SIZE) {
            map4k(
                high_range.start.wrapping_add(offset),
                kernel_range.start.wrapping_add(offset),
                Flags::PRESENT | Flags::READ_WRITE | Flags::GLOBAL
            ).expect("failed to map the kernel to a higher address range");
        }
    } else {
        panic!("kernel is too big (size: {}MiB, max: {}MiB), aborting",
            size / 1024 / 1024,
            max_size / 1024 / 1024
        );
    }

    let address = high_entry_point as *const ();
    let offset = (address as usize) - usize::from(kernel_range.start);

    let new_entry_point = core::mem::transmute::<_, unsafe extern "C" fn() -> !>(
        high_range.start.wrapping_add(offset)
    );

    new_entry_point();
}
*/
