#[macro_export]
macro_rules! io_write_port {
    (u8,  $port:expr, $value:expr) => { core::arch::asm!("out dx, al", in("dx") $port, in("al") $value as u8) };
    (u16, $port:expr, $value:expr) => { core::arch::asm!("out dx, ax", in("dx") $port, in("ax") $value as u16) };
    (u32, $port:expr, $value:expr) => { core::arch::asm!("out dx, eax", in("dx") $port, in("eax") $value as u32) };
}

#[macro_export]
macro_rules! io_read_port {
    (u8, $port:expr) => {
        let result: u8;
        core::arch::asm!("in dx, al", in("dx") $port, out("al") result);
        result
    };
}
