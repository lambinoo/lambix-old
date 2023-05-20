#[macro_export]
macro_rules! readmsr {
    ($addr:expr) => {{
        let eax: u32;
        let edx: u32;
        unsafe { core::arch::asm!("rdmsr" : "={edx}"(edx), "={eax}"(eax) : "{ecx}"($addr) :: "volatile") };
        [edx, eax]
    }}
}

#[macro_export]
macro_rules! writemsr {
    ($addr:expr, $regs:expr) => {
        core::arch::asm!("wrmsr" :: "{ecx}"($addr), "{edx}"($regs[0]), "{eax}"($regs[1]) :: "volatile")
    }
}

