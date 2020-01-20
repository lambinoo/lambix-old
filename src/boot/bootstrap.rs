use crate::kernel;
use crate::kernel_main;
use crate::kernel::config::*;
use crate::kernel::mem::paging::*;
use crate::kernel::mem::addr::*;

use ::lib::*;
use ::alloc::boxed::Box;

#[repr(align(4096))]
struct Page([u8; 12000]);

#[no_mangle]
pub unsafe extern "C" fn kernel_bootstrap() -> ! {
    disable_interrupts!();

    kernel::mem::setup_memory();
    kernel::idt::setup_idt();
    kernel::apic::setup_apic();

    exec_with_new_stack(kernel_main);
}

unsafe fn exec_with_new_stack(f: unsafe fn() -> !) -> ! {
    let stack_page: *mut Page = Box::into_raw(Box::new_zeroed().assume_init());
    asm!("movq $0, %rbp" :: "r"(stack_page));
    asm!("movq $0, %rsp" :: "r"(stack_page));
    enable_interrupts!();
    f();
}


/*
unsafe fn remap_kernel_and_jump(entry_point: unsafe fn() -> !) -> ! {
    let kernel_range = kernel::kernel_range();
    let high_range = VirtAddr::from(KERNEL_START) .. VirtAddr::from(KERNEL_END);

    let max_size = usize::from(high_range.end) - usize::from(high_range.start);
    let size = usize::from(kernel_range.end) - usize::from(kernel_range.start);

    if size < max_size {
        for offset in (0..size).step_by(PAGE_SIZE) {
            map4k(
                high_range.start.wrapping_add(offset),
                kernel_range.start.wrapping_add(offset),
                Flags::PRESENT | Flags::GLOBAL
            ).expect("failed to map the kernel to a higher address range");
        }
        purge_tlb();
    } else {
        panic!("kernel is too big (size: {}MiB, max: {}MiB), aborting",
            size / 1024 / 1024,
            max_size / 1024 / 1024
        );
    }

    let address = entry_point as *const ();
    let offset = (address as usize) - usize::from(kernel_range.start);

    let new_entry_point = core::mem::transmute::<_, fn() -> !>(
        high_range.start.wrapping_add(offset)
    );

    exec_with_new_stack(new_entry_point);
}
*/
