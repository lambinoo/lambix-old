#[macro_use]
pub mod vga_buffer;

extern {
    pub static multiboot_header_addr: u32;
}

