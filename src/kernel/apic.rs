pub mod registers;

use crate::kernel::mem::addr::*;
use crate::kernel::mem::vbox::*;

use lib::sync::*;
use lib::*;

use ::alloc::vec::Vec;
use core::convert::TryFrom;
use core::sync::atomic::*;

static APIC_REGS: StaticSpinlock<Vec<APIC>> = StaticSpinlock::new(Vec::new());

pub struct APIC {
    handle: VBox<APICRegisters>,
    cpu_hw_id: u8,
    is_bsc: bool,
}

impl APIC {
    pub fn get_id(&self) -> usize {
        usize::try_from(self.get32(APICRegister::ApicID).load(Ordering::SeqCst) >> 24).unwrap()
    }

    pub fn set_spurious_int_handler(&mut self, _vector: u8) {
        self.get32(APICRegister::SpuriousInterruptVector);
    }

    pub fn set_timer(&mut self, _count: usize, _periodic: bool) {}

    pub fn end_of_interrupt(&mut self) {
        self.get32(APICRegister::EndOfInterrupt)
            .store(0, Ordering::SeqCst);
    }
}

impl APIC {
    fn get32(&self, register: APICRegister) -> &AtomicU32 {
        &self.handle.registers[(register as usize) / core::mem::size_of::<APICRegister>()]
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
    SpecificEndOfInterrupt = 0x420,
}

#[repr(align(4096))]
struct APICRegisters {
    registers: [AtomicU32; 1024],
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
        usize::try_from(register[0] & APICRegisters::ADDR_MASK_HIGH).unwrap()
            | usize::try_from(register[1] & APICRegisters::ADDR_MASK_LOW).unwrap(),
    );

    let apic = APIC {
        handle: unsafe {
            VBox::with_flags(
                phy_addr,
                Flags::NO_EXECUTE | Flags::CACHE_DISABLE | Flags::WRITETHROUGH | Flags::READ_WRITE,
            )
            .unwrap()
        },
        cpu_hw_id: u8::try_from(cpuid!(0x1)[1] >> 24).unwrap(),
        is_bsc: (register[1] & APICRegisters::BSC_BIT) != 0,
    };

    APIC_REGS.lock().push(apic);

    let boot_info = crate::boot::multiboot::get_boot_info();
    let tag = boot_info
        .get_tag(crate::boot::multiboot::TagType::ACPIOldRsdp)
        .unwrap();

    let rdsp_ptr = tag.data();
}

#[inline]
fn disable_pic() {
    unsafe {
        io_write_port!(u8, 0xa1, 0xff);
        io_write_port!(u8, 0x21, 0xff);
    }
}
