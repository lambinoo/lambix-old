#[macro_export]
macro_rules! cpuid {
    ($addr:expr) => {{
        let eax : u32;
        let ebx : u32;
        let ecx : u32;
        let edx : u32;

        unsafe {
            core::arch::asm!("push rbx",
                             "cpuid",
                             "mov {:e}, ebx",
                             "pop rbx",
                             out(reg) ebx,
                             inout("eax") $addr => eax,
                             out("ecx") ecx,
                             out("edx") edx);
        };

        [eax, ebx, ecx, edx]
    }}
}
