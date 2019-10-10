use bitflags::bitflags;
use lib::disable_interrupts;

pub type Handler = unsafe extern "x86-interrupt" fn();

bitflags! {
    pub struct Flags: u8 {
        const PRESENT = 0b1000_0000;
        const TYPE_LDT = 0b0010;
        const TYPE_TSS = 0b1001;
        const TYPE_BUSYTSS = 0b1011;
        const TYPE_CALL = 0b1100;
        const TYPE_INTERRUPT = 0b1110;
        const TYPE_TRAP = 0b1111;
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct Segment {
    target_offset_low: u16,
    target_selector: u16,
    reserved0: u8,
    flags: u8,
    target_offset_mid: u16,
    target_offset_high: u32,
    reserved1: u32
}

impl Segment { 
    const fn new() -> Self {
        Segment {
            target_offset_low: 0,
            target_selector: 0,
            reserved0: 0,
            flags: 0,
            target_offset_mid: 0,
            target_offset_high: 0,
            reserved1: 0
        }
    }
}

impl Segment {
    fn set_handler(&mut self, handler: Handler) {
        let handler_ptr = handler as *const u8;
        let target_offset = handler_ptr as u64;
        self.target_offset_low = (target_offset & 0xFFFF) as u16;
        self.target_offset_mid = ((target_offset >> 16) & 0xFFFF) as u16;
        self.target_offset_high = ((target_offset >> 32) & 0xFFFF_FFFF) as u32;
    }

    fn set_selector(&mut self, offset_gdt: u16) {
        self.target_selector = offset_gdt;
    }

    fn set_flags(&mut self, flags: Flags) {
        self.flags = flags.bits;
    }
}


#[repr(C)]
pub struct InterruptDescriptorTable {
    table: [Segment; 256]
}

impl InterruptDescriptorTable {
    pub const fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable { table: [Segment::new(); 256] }
    }

    pub fn set_handler(&mut self, int: u8, flags: Flags, handler: Handler) {
        let segment = &mut self.table[int as usize];
        segment.set_handler(handler);
        segment.set_flags(flags);
        segment.set_selector(0x20);
    }

    pub unsafe fn load(&self) {
        let register = IDTRegister::new(self);
        disable_interrupts!();
        asm!("lidt ($0)" :: "r" (&register) : "memory");
    }
}

#[repr(packed)]
#[allow(unused)]
struct IDTRegister {
    size: u16,
    addr: u64
}

impl IDTRegister {
    unsafe fn new(idt: &InterruptDescriptorTable) -> IDTRegister {
        IDTRegister {
            addr: (idt as *const _) as u64,
            size: core::mem::size_of::<InterruptDescriptorTable>() as u16
        }
    }
}

