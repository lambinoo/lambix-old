use core::convert::From;
use core::convert::TryFrom;

#[derive(Debug)]
pub enum Vector {
    DivideByZero,
    Debug,
    NMI,
    Breakpoint,
    Overflow,
    BoundRange,
    InvalidOpCode,
    DeviceNotAvailable,
    X87FloatingPoint,
    MachineCheck,
    SIMDFloatingPoint,
    VMMCommunication,
    Security,
}

impl From<Vector> for usize {
    fn from(vector: Vector) -> usize {
        match vector {
            Vector::DivideByZero => 0,
            Vector::Debug => 1,
            Vector::NMI => 2,
            Vector::Breakpoint => 3,
            Vector::Overflow => 4,
            Vector::BoundRange => 5,
            Vector::InvalidOpCode => 6,
            Vector::DeviceNotAvailable => 7,
            Vector::X87FloatingPoint => 16,
            Vector::MachineCheck => 18,
            Vector::SIMDFloatingPoint => 19,
            Vector::VMMCommunication => 29,
            Vector::Security => 30,
        }
    }
}

impl TryFrom<usize> for Vector {
    type Error = ();
    fn try_from(vector: usize) -> Result<Vector, ()> {
        match vector {
            0 => Ok(Vector::DivideByZero),
            1 => Ok(Vector::Debug),
            2 => Ok(Vector::NMI),
            3 => Ok(Vector::Breakpoint),
            4 => Ok(Vector::Overflow),
            5 => Ok(Vector::BoundRange),
            6 => Ok(Vector::InvalidOpCode),
            7 => Ok(Vector::DeviceNotAvailable),
            16 => Ok(Vector::X87FloatingPoint),
            18 => Ok(Vector::MachineCheck),
            19 => Ok(Vector::SIMDFloatingPoint),
            29 => Ok(Vector::VMMCommunication),
            30 => Ok(Vector::Security),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub enum VectorWithError {
    DoubleFault,
    InvalidTSS,
    SegmentNotPresent,
    Stack,
    GeneralProtection,
    PageFault,
    AlignmentCheck,
}

impl From<VectorWithError> for usize {
    fn from(vector: VectorWithError) -> usize {
        match vector {
            VectorWithError::DoubleFault => 8,
            VectorWithError::InvalidTSS => 10,
            VectorWithError::SegmentNotPresent => 11,
            VectorWithError::Stack => 12,
            VectorWithError::GeneralProtection => 13,
            VectorWithError::PageFault => 14,
            VectorWithError::AlignmentCheck => 17,
        }
    }
}

impl TryFrom<usize> for VectorWithError {
    type Error = ();
    fn try_from(vector: usize) -> Result<VectorWithError, ()> {
        match vector {
            8 => Ok(VectorWithError::DoubleFault),
            10 => Ok(VectorWithError::InvalidTSS),
            11 => Ok(VectorWithError::SegmentNotPresent),
            12 => Ok(VectorWithError::Stack),
            13 => Ok(VectorWithError::GeneralProtection),
            14 => Ok(VectorWithError::PageFault),
            17 => Ok(VectorWithError::AlignmentCheck),
            _ => Err(()),
        }
    }
}
