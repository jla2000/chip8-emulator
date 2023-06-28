pub fn nibbles(value: u16) -> (u8, u8, u8, u8) {
    (
        ((value & 0xf000) >> 12) as u8,
        ((value & 0x0f00) >> 8) as u8,
        ((value & 0x00f0) >> 4) as u8,
        (value & 0x000f) as u8,
    )
}
