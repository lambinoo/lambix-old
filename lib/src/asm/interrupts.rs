#[macro_export]
macro_rules! int {
    ($vector:expr) => {
        asm!("int $0" :: "N"($vector) :: "volatile")
    }
}

#[macro_export]
macro_rules! enable_interrupts {
    () => { asm!("sti" :::: "volatile") }
}

#[macro_export]
macro_rules! disable_interrupts {
    () => { asm!("cli" :::: "volatile") }
}

#[macro_export]
macro_rules! isr {
    (
        $($v:vis fn $name:ident($($arg:ident: $argtype:ty),*) {
            $($body:tt)*
        })*
    ) => {
        #[allow(unreachable_code)]
        $($v unsafe extern "x86-interrupt" fn $name($($arg: $argtype),*) {
            {
                let f = move || {
                    $($body)*
                };

                use lib::{enable_interrupts, disable_interrupts};
                disable_interrupts!();
                f();
                enable_interrupts!();
            }
        })*
    };
}

