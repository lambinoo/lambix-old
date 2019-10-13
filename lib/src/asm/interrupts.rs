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
        use lib::{enable_interrupts, disable_interrupts};

        $(unsafe extern "x86-interrupt" fn $name() {
            disable_interrupts!();
            $($body)*
            enable_interrupts!();
        })*
    };
}

