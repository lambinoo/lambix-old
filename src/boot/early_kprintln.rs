use core::fmt::{Arguments, Result, Write};
use lib::*;

struct IOPort(u32);

impl Write for IOPort {
    fn write_str(&mut self, buf: &str) -> Result {
        for c in buf.chars() {
            unsafe {
                io_write_port!(u8, 0x3F8, c);
            }
        }

        Ok(())
    }
}

pub fn write_to_serial(args: Arguments) {
    let mut port = IOPort(0x3F8);
    port.write_fmt(args)
        .expect("failed to write to serial port");
}

#[macro_export]
macro_rules! early_kprint {
    ($($arg:tt)*) => (
        {
            $crate::boot::early_kprintln::write_to_serial(format_args!( $($arg)* ));
            $crate::drivers::vga_buffer::_print(format_args!( $($arg)* ));
        }
    )
}

#[macro_export]
macro_rules! early_kprintln {
    () => ( early_kprint!("\n") );
    ($($arg:tt)*) => (
        {
            $crate::boot::early_kprintln::write_to_serial(format_args_nl!( $($arg)* ));
            $crate::drivers::vga_buffer::_print(format_args_nl!( $($arg)* ));
        }
    )
}
