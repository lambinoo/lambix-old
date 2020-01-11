use core::marker::PhantomData;

#[repr(C)]
pub struct Entry<T> {
    entry: EntryInner,
    t: PhantomData<T>
}

#[repr(C)]
pub struct EntryInner {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32
}

