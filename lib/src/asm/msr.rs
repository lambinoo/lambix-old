#[macro_export]
macro_rules! readmsr {
    ($addr:expr) => {{
        let eax: u32;
        let edx: u32;
        unsafe { asm!("rdmsr" : "={edx}"(edx), "={eax}"(eax) : "{ecx}"($addr) :: "volatile") };
        [edx, eax]
    }}
}

#[macro_export]
macro_rules! writemsr {
    ($addr:expr, $regs:expr) => {
        asm!("wrmsr" :: "{ecx}"($addr), "{edx}"($regs[0]), "{eax}"($regs[1]) :: "volatile")
    }
}

