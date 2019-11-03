use lib::*;
use lib::sync::*;
use core::ptr::*;

pub static APIC: Spinlock<APICRegisters> = Spinlock::new(APICRegisters { base_addr: None });
pub type Result<T> = core::result::Result<T, ()>;

macro_rules! set_registers {
    ($(
        ($name:ident, $register:expr, $type:ty)
    )*) => {
        $(
            pub unsafe fn $name(&mut self, value: $type) -> Result<()> {
                self.set_register($register, value)
            }
        )*
    }
}

#[repr(usize)]
enum APICRegister {
    APICID = 0x20,
    APICVersion = 0x30,
    TaskPriority = 0x80,
    SpuriousInterruptVector = 0xf0,
    TriggerMode = 0x200,
    ErrorStatus = 0x280,
    TimerLocalVector = 0x320
}

pub struct APICRegisters {
    base_addr: Option<NonNull<u8>>
}

impl APICRegisters {
    const MSR_APIC: u32 = 0x0000_001B;
    const CPUID_REGISTER: u32 = 0x1;
    const CPUID_APIC_AVAILABLE_FLAG: u32 = 1 << 9;

    const IS_BOOT_CPU: u32 = 1 << 8;
    const APIC_ENABLE: u32 = 1 << 11;

    const MASK_ADDR1: u64 = 0xfffff000;
    const MASK_ADDR0: u64 = 0xfffff;

    set_registers!{
        (set_local_timer, APICRegister::TimerLocalVector, u32)
        (set_spurious_interrupt, APICRegister::SpuriousInterruptVector, u32)
    }

    pub fn is_available(&self) -> bool {
        self.base_addr.is_some()
    }

    fn get_register<T>(&self, offset: APICRegister) -> Result<T> {
        if let Some(base_addr) = self.base_addr {
            unsafe {
                let register = base_addr.as_ptr().add(offset as usize) as *const T;
                Ok(register.read_volatile())
            }
        } else {
            Err(())
        }
    }

    unsafe fn set_register<T>(&mut self, offset: APICRegister, value: T) -> Result<()> {
        if let Some(base_addr) = self.base_addr {
            let register = base_addr.as_ptr().add(offset as usize) as *mut T;
            register.write_volatile(value);
            Ok(())
        } else {
            Err(())
        }
    }
}

pub fn setup_apic() -> Result<()> {
    if is_apic_supported_by_cpu() {
        disable_pic();  

        let mut apic_reg = readmsr!(APICRegisters::MSR_APIC);

        // just an address for development purposes
        let pic_addr = 0x0010_0000u64;
        apic_reg[0] = ((pic_addr >> 32) & APICRegisters::MASK_ADDR0) as u32;
        apic_reg[1] = (pic_addr & APICRegisters::MASK_ADDR1) as u32;
        apic_reg[1] |= APICRegisters::IS_BOOT_CPU;
        apic_reg[1] |= APICRegisters::APIC_ENABLE;

        unsafe {
            writemsr!(APICRegisters::MSR_APIC, apic_reg)
        };

        assert!(readmsr!(APICRegisters::MSR_APIC) == apic_reg);

        if let Some(pic_addr) = NonNull::new(pic_addr as _) {
            APIC.lock().base_addr = Some(pic_addr);
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

fn is_apic_supported_by_cpu() -> bool {
    let cap: u32;
    unsafe {
        asm!("cpuid"
            :"={edx}"(cap)
            :"{eax}"(APICRegisters::CPUID_REGISTER))
    };

    cap & APICRegisters::CPUID_APIC_AVAILABLE_FLAG != 0
}

#[inline]
fn disable_pic() {
    unsafe {
        io_write_port!(u8, 0xa1, 0xff);
        io_write_port!(u8, 0x21, 0xff);
    }
}

