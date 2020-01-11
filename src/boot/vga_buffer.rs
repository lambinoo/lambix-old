mod buffer;
mod character;
mod line;

pub use buffer::VGABuffer;
pub use buffer::CharacterBuffer;
pub use character::Character;
pub use line::Line;

use crate::kernel::mem::addr::PhyAddr;

use core::fmt::{Arguments, Write};
use core::ptr::NonNull;

use lib::sync::Spinlock;

pub const BASE_ADDR: PhyAddr = PhyAddr::new(0xb8000);
pub static VGA_BUFFER: Spinlock<VGABuffer> = Spinlock::new(unsafe { 
    VGABuffer::new(NonNull::new_unchecked(BASE_ADDR.as_mut_ptr()))
});

#[inline]
pub fn _print(args: Arguments) {
    let result = VGA_BUFFER.lock().write_fmt(args);
    result.expect("early_printk failed when trying to write to vga text buffer")
}

