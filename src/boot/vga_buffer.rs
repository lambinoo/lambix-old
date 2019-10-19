mod buffer;
mod character;

pub use buffer::VGABuffer;
pub use buffer::VGA_BUFFER_ADDR;
pub use buffer::CharacterBuffer;
pub use character::Character;

use core::fmt::{Arguments, Write, self};
use lib::sync::Mutex;

static VGA_BUFFER: Mutex<VGABuffer> = unsafe { Mutex::new(VGABuffer::new(VGA_BUFFER_ADDR as _)) };

pub fn _try_print(args: Arguments) -> fmt::Result {
    VGA_BUFFER.lock().write_fmt(args)
}

#[inline]
pub fn _print(args: Arguments) {
    _try_print(args)
        .expect("early_printk failed when trying to write to vga text buffer")
}

