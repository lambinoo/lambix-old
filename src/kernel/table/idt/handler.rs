use core::fmt::Debug;

pub type Handler = unsafe extern "x86-interrupt" fn(&InterruptStackFrame);
pub type HandlerWithError = unsafe extern "x86-interrupt" fn(&InterruptStackFrame, u64);

#[repr(C)]
#[derive(Debug)]
pub struct InterruptStackFrame {
    rip: *const (),
    cs: usize,
    rflags: *const (),
    rsp: *const (),
    ss: usize
}

