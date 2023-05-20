#[macro_export]
macro_rules! cpuid {
    ($addr:expr) => {{
        let eax : u32;
        let ebx : u32;
        let ecx : u32;
        let edx : u32;

        unsafe {
            core::arch::asm!("push ebx",
                             "cpuid",
                             "mov {ebx_tmp}, ebx",
                             "pop ebx",
                             out("eax") eax,
                             ebx_tmp = out(reg) ebx,
                             out("ecx") ecx,
                             out("edx") edx);
        };

        [eax, ebx, ecx, edx]
    }}
}
