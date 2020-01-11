use core::ops::{ Index, IndexMut };
use core::mem::transmute;
use core::mem::MaybeUninit;
use super::*;

pub struct IDT {
    entries: [EntryInner; 256]
}

impl IDT {
    pub fn new() -> IDT {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}


impl Index<Vector> for IDT {
    type Output = Entry<Handler>;
    fn index(&self, int: Vector) -> &Self::Output {
        unsafe { transmute(&self.entries[usize::from(int)]) }
    }
}

impl IndexMut<Vector> for IDT {
    fn index_mut(&mut self, int: Vector) -> &mut Self::Output {
        unsafe { transmute(&mut self.entries[usize::from(int)]) }
    }
}


impl Index<VectorWithError> for IDT {
    type Output = Entry<HandlerWithError>;
    fn index(&self, int: VectorWithError) -> &Self::Output {
        unsafe { transmute(&self.entries[usize::from(int)]) }
    }
}

impl IndexMut<VectorWithError> for IDT {
    fn index_mut(&mut self, int: VectorWithError) -> &mut Self::Output {
        unsafe { transmute(&mut self.entries[usize::from(int)]) }
    }
}


impl Index<usize> for IDT {
    type Output = Entry<Handler>;
    fn index(&self, int: usize) -> &Self::Output {
        if int > 31 { 
            unsafe { transmute(&self.entries[int]) }
        } else {
            panic!("All interrupts below 32 are reserved and can't be assigned manually. Panicking.");
        }
    }
}

impl IndexMut<usize> for IDT {
    fn index_mut(&mut self, int: usize) -> &mut Self::Output {
        if int > 31 {
            unsafe { transmute(&mut self.entries[int]) }
        } else { 
            panic!("All interrupts below 32 are reserved and can't be assigned manually. Panicking.");
        }
    }
}

