#[macro_export]
macro_rules! cpuid {
    ($addr:expr) => {{
        let eax : u32;
        let ebx : u32;
        let ecx : u32;
        let edx : u32;

        unsafe {
            asm!("cpuid"
                : "={eax}"(eax), "={ebx}"(ebx), "={ecx}"(ecx), "={edx}"(edx)
                : "{eax}"($addr));
        };
        
        [eax, ebx, ecx, edx]
    }}
}

