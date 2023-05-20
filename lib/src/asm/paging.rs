#[macro_export]
macro_rules! set_cr3 {
    ($value:expr) => {
        let value = $value;
        core::arch::asm!("mov $0, %cr3" :: "r"(value) :: "volatile");
    }
}

#[macro_export]
macro_rules! get_cr3 {
    () => {
        {
            let cr3: usize;
            unsafe { core::arch::asm!("mov %cr3, $0" : "=r"(cr3) ::: "volatile") }
            cr3
        }
    }
}
