use crate::kernel::mem::addr::*;
use crate::kernel::mem::vbox::*;

use lib::*;
use lib::sync::*;

use core::convert::TryFrom;
use core::sync::atomic::*;
use ::alloc::vec::Vec;

static APIC_REGS: Spinlock<Vec<APIC>> = Spinlock::new(Vec::new());

pub struct APIC {
    handle: VBox<APICRegisters>,
    cpu_hw_id: u8,
    is_bsc: bool
}

impl APIC {
    fn get32(&self, register: APICRegister) -> &AtomicU32 {
        &self.handle.registers[(register as usize) / 4]
    }

    fn get64(&self, register: APICRegister) -> &AtomicU64 {
        unsafe { core::mem::transmute(self.get32(register)) }
    }
}

#[repr(usize)]
enum APICRegister {
    ApicID = 0x20,
    ApicVersion = 0x30,
    TaskPriority = 0x80,
    ArbitrationPriority = 0x90,
    ProcessorPriority = 0xa0,
    EndOfInterrupt = 0xb0,
    RemoteRead = 0xc0,
    LogicalDestination = 0xd0,
    DestinationFormat = 0xe0,
    SpuriousInterruptVector = 0xf0,
    InService = 0x100,
    TriggerMode = 0x180,
    InterruptRequest = 0x200,
    ErrorStatus = 0x280,
    InterruptCommandLow = 0x300,
    InterruptCommandHigh = 0x310,
    TimerLocalVectorTable = 0x320,
    ThermalLocalVector = 0x330,
    PerformanceCounterLocalVectorTable = 0x340,
    LocalInterrupt0VectorTable = 0x350,
    LocalInterrupt1VectorTable = 0x360,
    ErrorVectorTable = 0x370,
    TimerInitialCount = 0x380,
    TimerCurrentCount = 0x390,
    TimerDivideConfiguration = 0x3e0,
    ExtendedAPICFeature = 0x400,
    ExtendedAPICControl = 0x410,
    SpecificEndOfInterrupt = 0x420
}


#[repr(align(4096))]
struct APICRegisters {
    registers: [AtomicU32; 1024]
}

impl APICRegisters {
    const MSR_APIC_BASE_ADDR: usize = 0x1b;
    const BSC_BIT: u32 = 8;
    const APIC_ENABLE_BIT: u32 = 1 << 11;
    const ADDR_MASK_HIGH: u32 = (1 << 20) - 1;
    const ADDR_MASK_LOW: u32 = !((1 << 12) - 1);
}


pub fn setup_apic() {
    disable_pic();

    let mut register = readmsr!(APICRegisters::MSR_APIC_BASE_ADDR);
    register[1] |= APICRegisters::APIC_ENABLE_BIT;
    unsafe {
        writemsr!(APICRegisters::MSR_APIC_BASE_ADDR, register);
    };
    
    let phy_addr = PhyAddr::new(
        usize::try_from(register[0] & APICRegisters::ADDR_MASK_HIGH).unwrap() |
        usize::try_from(register[1] & APICRegisters::ADDR_MASK_LOW).unwrap()
    );

    let apic = APIC {
        handle: unsafe { VBox::with_flags(phy_addr, Flags::NO_EXECUTE | Flags::CACHE_DISABLE | Flags::WRITETHROUGH | Flags::READ_WRITE) },
        cpu_hw_id: u8::try_from(cpuid!(0x1)[1] >> 24).unwrap(),
        is_bsc: (register[1] & APICRegisters::BSC_BIT) != 0
    };

/*
    apic.get32(APICRegister::SpuriousInterruptVector).store(150 | (1 << 8), Ordering::SeqCst);
    apic.get32(APICRegister::TaskPriority).store(0, Ordering::SeqCst);
    apic.get32(APICRegister::TimerDivideConfiguration).store(0b1011, Ordering::SeqCst);
    apic.get32(APICRegister::TimerLocalVectorTable).store(200 | (1 << 17), Ordering::SeqCst);
    apic.get32(APICRegister::TimerInitialCount).store(1, Ordering::SeqCst);*/

    APIC_REGS.lock().push(apic);
}

#[inline]
fn disable_pic() {
    unsafe {
        io_write_port!(u8, 0xa1, 0xff);
        io_write_port!(u8, 0x21, 0xff);
    }
}

