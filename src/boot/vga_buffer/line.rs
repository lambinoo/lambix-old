use super::Character;
use super::buffer::COLUMNS;

#[repr(transparent)]
pub struct Line {
    chars: [Character; COLUMNS]
}

impl Line {
    pub fn get_char(&self, idx: usize) -> Character {
        let ptr = &self.chars[idx] as *const Character;
        unsafe { ptr.read_volatile() }
    }
 
    pub fn set_char(&mut self, idx: usize, character: Character) {
        let ptr = &mut self.chars[idx] as *mut Character;
        unsafe { ptr.write_volatile(character) }
    }

    pub fn set(&mut self, idx: usize, character: u8, color: u8) {
        self.set_char(idx, Character::new(character, color));
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

