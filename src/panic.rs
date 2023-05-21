use core::panic::PanicInfo;

#[cold]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    early_kprintln!("\x1B\x74----- [PANIC START HERE] -----");
    early_kprint!("kernel panicked ");

    if let Some(location) = info.location() {
        early_kprintln!("at {}:{}", location.file(), location.line());
    } else {
        early_kprintln!("at <unknown>");
    }

    if let Some(message) = info.message() {
        early_kprintln!("Reason: {:?}", message);
    }

    early_kprintln!("----- [PANIC  END  HERE] -----\x1B\0");

    loop {}
}
