#[macro_export]
macro_rules! readmsr {
    ($addr:expr) => {{
        let eax: u32;
        let edx: u32;
        unsafe { core::arch::asm!("rdmsr", in("ecx") $addr, out("edx") edx, out("eax") eax) }
        [edx, eax]
    }}
}

#[macro_export]
macro_rules! writemsr {
    ($addr:expr, $regs:expr) => {
        unsafe { core::arch::asm!("wrmsr", in("ecx") $addr, in("edx") $regs[0], in("eax") $regs[1]) } 
    }
}
