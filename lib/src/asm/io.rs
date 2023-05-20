#[macro_export]
macro_rules! io_write_port {
    (u8,  $port:expr, $value:expr) => { core::arch::asm!("out %al, %dx"  :: "{dx}"($port), "{al}"($value)  :: "volatile") };
    (u16, $port:expr, $value:expr) => { core::arch::asm!("out %ax, %dx"  :: "{dx}"($port), "{ax}"($value)  :: "volatile") };
    (u32, $port:expr, $value:expr) => { core::arch::asm!("out %eax, %dx" :: "{dx}"($port), "{eax}"($value) :: "volatile") };
}

#[macro_export]
macro_rules! io_read_port {
    (u8, $port:expr) => {
        let result: u8;
        core::arch::asm!("in %dx, %al" : "={al}"($result) : "{dx}"($port) :: "volatile");
        result
    }; 
}

