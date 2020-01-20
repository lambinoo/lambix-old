#[repr(C)]
#[derive(Clone, Copy)]
pub struct Character {
    character: u8,
    color: u8
}

impl Character {
    pub fn new(character: u8, color: u8) -> Character {
        Character { character, color }
    }
}

