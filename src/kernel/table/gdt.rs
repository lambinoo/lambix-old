struct Segment {
    segment_limit_low: u16,
    base_addr_low: u16,
    base_addr_midlow: u8,
    flags0: u8,
    segment_limit_high_and_flags1: u8,
    base_addr_midup: u8,
    base_addr_up: u32,
    zero: u32
}

