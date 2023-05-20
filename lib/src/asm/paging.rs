#[macro_export]
macro_rules! set_cr3 {
    ($value:expr) => {
        core::arch::asm!("mov cr3, {}", in(reg) $value);
    }
}

#[macro_export]
macro_rules! get_cr3 {
    () => {
        {
            let cr3: usize;
            core::arch::asm!("mov {}, cr3", out(reg) cr3);
            cr3
        }
    }
}
