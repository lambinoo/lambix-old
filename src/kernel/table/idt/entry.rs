use core::marker::PhantomData;
use core::convert::TryInto;
use super::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Entry<T> {
    entry: EntryInner,
    t: PhantomData<T>
}

impl<T> Entry<T> {
    const PRESENT_BIT: u8 = 1 << 7;

    fn set_raw(&mut self, handler: usize, gate_type: GateType, dpl: DPL, selector: u16) {
        self.entry.offset_low = (handler & 0xffff).try_into().unwrap();
        self.entry.offset_mid = ((handler >> 16) & 0xffff).try_into().unwrap();
        self.entry.offset_high = ((handler >> 32) & 0xffffffff).try_into().unwrap();
        self.entry.selector = selector;
        self.entry.ist = 0;
        self.entry.flags = Self::PRESENT_BIT | (dpl as u8) | (gate_type as u8);
    }

    pub fn clear(&mut self) {
        self.entry.flags = 0;
    }
}

impl Entry<Handler> {
    pub fn set(&mut self, handler: Handler, gate_type: GateType, dpl: DPL, selector: u16) {
        self.set_raw(handler as usize, gate_type, dpl, selector);
    }
}

impl Entry<HandlerWithError> {
    pub fn set(&mut self, handler: HandlerWithError, gate_type: GateType, dpl: DPL, selector: u16) {
        self.set_raw(handler as usize, gate_type, dpl, selector);
    }
}


#[repr(C)]
#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct EntryInner {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32
}

#[repr(u8)]
pub enum DPL {
    PRIVILEGE0 = 0,
    PRIVILEGE1 = 1 << 5,
    PRIVILEGE2 = 2 << 5,
    PRIVILEGE3 = 3 << 5,
}

#[repr(u8)]
pub enum GateType {
    INTERRUPT = 0xe,
    TRAP = 0xf
}

