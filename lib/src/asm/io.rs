#[macro_export]
macro_rules! io_write_port {
    (u8,  $port:expr, $value:expr) => { asm!("out %al, %dx"  :: "{dx}"($port), "{al}"($value)  :: "volatile") };
    (u16, $port:expr, $value:expr) => { asm!("out %ax, %dx"  :: "{dx}"($port), "{ax}"($value)  :: "volatile") };
    (u32, $port:expr, $value:expr) => { asm!("out %eax, %dx" :: "{dx}"($port), "{eax}"($value) :: "volatile") };
}

#[macro_export]
macro_rules! io_read_port {
    (u8, $port:expr) => {
        let result: u8;
        asm!("in %dx, %al" : "={al}"($result) : "{dx}"($port) :: "volatile");
        result
    }; 
}

