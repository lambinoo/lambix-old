use lib::*;
use crate::kernel;
use crate::kernel::idt::*;
use crate::kernel::table::idt::*;
use crate::kernel_main;
use crate::kernel::mem::addr::*;
use crate::kernel::mem::vbox::VBox;

use core::convert::TryFrom;
use core::sync::atomic::*;

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
    registers: [AtomicU32; 1024]
}


static TIMER_COUNTER: AtomicUsize = AtomicUsize::new(0);

isr! {
    fn timer_handler() {
        TIMER_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

unsafe fn setup_apic() {
    io_write_port!(u8, 0xa1, 0xff);
    io_write_port!(u8, 0x21, 0xff);
    enable_interrupts!();

    let apic_msr_reg = 0x1b;
    let mut apic_addr_reg = readmsr!(apic_msr_reg);
    apic_addr_reg[1] |= 1 << 11;
    writemsr!(apic_msr_reg, apic_addr_reg);

    early_kprintln!("{:?}", apic_addr_reg);

    let mask: u64 = !(0xfff | (0xfff << 52));
    let apic_base_addr = PhyAddr::from(usize::try_from(
        (u64::from(apic_addr_reg[1]) | u64::from(apic_addr_reg[0]) << 32)
        & mask
    ).unwrap());
    

    let apic: VBox<APICRegisters> = VBox::new(apic_base_addr);
    GLOBAL_IDT.lock().set_handler(
        200,
        Flags::PRESENT | Flags::TYPE_INTERRUPT,
        timer_handler
    );
    

    let spurious_reg = &apic.registers[0xf0 / 4];
    let timer_reg = &apic.registers[0x320 / 4];
    let timer_diviser = &apic.registers[0x3E0 / 4];
    let timer_count = &apic.registers[0x380 / 4];
    let task_priority = &apic.registers[0x80 / 4];
    let end_of_int = &apic.registers[0xb0 / 4];

    spurious_reg.store((1 << 8) | 0xff, Ordering::SeqCst);
    task_priority.store(0, Ordering::SeqCst);
    timer_diviser.store(0b1011, Ordering::SeqCst);
    timer_reg.store((1 << 17) | 200, Ordering::SeqCst);
    timer_count.store(1, Ordering::SeqCst);
  
    let mut previous = 0;
    loop {
        let new_value = TIMER_COUNTER.load(Ordering::Relaxed);
        if previous != new_value {
            previous = new_value;
            end_of_int.store(0, Ordering::SeqCst);

            if new_value % 1000 == 0 {
                early_kprintln!("{}", new_value);
            }
        }
    }
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
