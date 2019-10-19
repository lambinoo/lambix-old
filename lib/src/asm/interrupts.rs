#[macro_export]
macro_rules! int {
    ($vector:expr) => {
        asm!("int $0" :: "N"($vector))
    }
}

#[macro_export]
macro_rules! enable_interrupts {
    () => { asm!("sti") }
}

#[macro_export]
macro_rules! disable_interrupts {
    () => { asm!("cli") }
}

#[macro_export]
macro_rules! isr {
    (
        $(fn $name:ident() {
            $($body:tt)*
        })*
    ) => {
        $(unsafe extern "x86-interrupt" fn $name() {
            {
                use lib::{enable_interrupts, disable_interrupts};
                disable_interrupts!();
                $($body)*
                enable_interrupts!();
            }
        })*
    };
}

