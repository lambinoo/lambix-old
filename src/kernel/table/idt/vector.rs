use core::convert::From;

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
    Security
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
            Vector::Security => 30
        }
    }
}


pub enum VectorWithError {
    DoubleFault,
    InvalidTSS,
    SegmentNotPresent,
    Stack,
    GeneralProtection,
    PageFault,
    AlignmentCheck
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
            VectorWithError::AlignmentCheck => 17
        }
    }
}

