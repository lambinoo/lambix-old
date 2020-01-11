use core::marker::PhantomData;
use core::fmt::{Write, Result};
use core::ptr::NonNull;
use super::Line;

pub const COLUMNS: usize = 80;
pub const LINES: usize = 25;

pub type CharacterBuffer = [Line; LINES];

pub struct VGABuffer<'b> {
    buffer: NonNull<CharacterBuffer>,
    column: usize,
    current_color: u8,
    line_sizes: [usize; LINES],
    _phantom: PhantomData<&'b mut CharacterBuffer>
}

impl<'b> VGABuffer<'b> {
    /// Create a new VGABuffer to manage a buffer
    ///
    /// # Unsafe
    /// buffer must point to a valid vga buffer
    pub const unsafe fn new(buffer: NonNull<CharacterBuffer>) -> VGABuffer<'b> {
        let vga_buffer = VGABuffer {
            buffer,
            column: 0,
            current_color: 7,
            line_sizes: [0; LINES],
            _phantom: PhantomData
        };
        vga_buffer 
    }

    pub fn clear(&mut self) {
        let buffer = unsafe { self.buffer.as_mut() };
        for i in 0..buffer.len() {
            self.line_sizes[i] = 0;
            for j in 0..buffer[i].len() {
                buffer[i].set(j, 0, 0);
            }
        }
    }

    pub fn set_color(&mut self, color: u8) {
        self.current_color = color;
    }

    /// Change the internal buffer address (in case it's remapped for example)
    ///
    /// # Unsafe
    /// See `VGABuffer::new()` constraints.
    pub unsafe fn set_buffer_addr(&mut self, addr: NonNull<CharacterBuffer>) {
        *self = VGABuffer::new(addr);
        self.clear();
    }

    fn move_all_lines_up(&mut self) {
        unsafe {
            for i in 1..LINES {
                let lines = self.buffer.as_mut();

                for j in 0..self.line_sizes[i] {
                    lines[i-1].set_char(j, lines[i].get_char(j));
                }

                for j in self.line_sizes[i]..self.line_sizes[i-1] {
                    lines[i-1].set(j, b' ', 0);
                }

                self.line_sizes[i-1] = self.line_sizes[i];
            }
        }
    }

    fn new_line(&mut self) {
        self.move_all_lines_up();
        self.clear_last_line();
        self.column = 0;
    }

    fn write_byte(&mut self, character: u8) {
        let last_line = unsafe { self.buffer.as_mut().last_mut().unwrap() };
        last_line.set(self.column, character, self.current_color);

        self.column += 1;
        self.line_sizes[LINES - 1] = self.column;
        if self.column == last_line.len() {
            self.new_line();
        }
    }

    fn clear_last_line(&mut self) {
        let line = unsafe { self.buffer.as_mut().last_mut().unwrap() };
        for i in 0..self.line_sizes[LINES - 1] {
            line.set(i, b' ', 0);
        }
        self.line_sizes[LINES - 1] = 0;
    } 
}

impl<'b> Write for VGABuffer<'b> {
    fn write_str(&mut self, s: &str) -> Result {
        let mut chars = s.chars();
        while let Some(c) = chars.next(){
            if c.is_ascii() {
                match c {
                    '\n' => self.new_line(),
                    '\x1B' => {
                        if let Some(color) = chars.next() {
                            self.set_color(color as u8);
                        }
                    }
                    c => self.write_byte(c as u8)
                }
            }
        }
        Ok(())
    }
}

