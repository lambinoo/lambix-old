use core::fmt::{Write, Result};
use super::Character;

pub type CharacterLine = [Character; 80];
pub type CharacterBuffer = [CharacterLine; 25];

pub const VGA_BUFFER_ADDR: *mut CharacterBuffer = 0xb8000 as _;

#[derive(Debug)]
pub struct VGABuffer {
    ptr: *mut CharacterBuffer,
    column: usize,
    current_color: u8
}

fn _ref<'a, T>(ptr: *mut T) -> &'a mut T {
    unsafe { &mut *ptr }
}

impl VGABuffer {
    pub const unsafe fn new(addr: *mut CharacterBuffer) -> VGABuffer {
        VGABuffer {
            ptr: addr,
            column: 0,
            current_color: 7
        }
    } 

    pub fn clear(&mut self) {
        for line in _ref(self.ptr).iter_mut() {
            for c in line.iter_mut() {
                c.character = b' ';
                c.color = 0;
            }
        }
    }

    pub fn set_color(&mut self, color: u8) {
        self.current_color = color;
    }

    fn move_all_lines_up(&mut self) {
        let buffer = _ref(self.ptr);
        for i in 1..buffer.len() {
            buffer[i-1] = buffer[i].clone();
        }
    }

    fn new_line(&mut self) {
        self.move_all_lines_up();
        self.clear_last_line();
        self.column = 0;
    }

    fn write_byte(&mut self, character: u8) {
        let last_line = _ref(self.ptr).last_mut().unwrap();
        let ptr = (&mut last_line[self.column]) as *mut Character;
        unsafe {
             core::ptr::write_volatile(ptr, Character {
                color: self.current_color,
                character
             });
        };

        self.column += 1;
        if self.column == last_line.len() {
            self.new_line();
        }
    }

    fn clear_last_line(&mut self) {
        let line = &mut _ref(self.ptr).last_mut().unwrap();
        for c in line.iter_mut() {
            c.character = b' ';
            c.color = 0;
        }
    }
}

impl Write for VGABuffer {
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

