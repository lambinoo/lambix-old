pub type Handler = unsafe extern "x86-interrupt" fn(&InterruptStackFrame);
pub type HandlerWithError = unsafe extern "x86-interrupt" fn(&InterruptStackFrame, u64);

#[repr(C)]
pub struct InterruptStackFrame {
    rip: usize,
    cs: usize,
    rflags: usize,
    rsp: usize,
    ss: usize
}

