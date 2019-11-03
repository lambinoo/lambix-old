#[macro_export]
macro_rules! set_cr3 {
    ($value:expr) => {
        let value = $value;
        asm!("mov $0, %cr3" :: "r"(value) :: "volatile");
    }
}

#[macro_export]
macro_rules! get_cr3 {
    () => {
        {
            let cr3: usize;
            unsafe { asm!("mov %cr3, $0" : "=r"(cr3) ::: "volatile") }
            cr3
        }
    }
}

