#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Storage {
    Bits1,  // Terminal defaults
    Bits4,  // 4-bit (Standard)
    Bits8,  // 8-bit
    Bits24, // 24-bit (True Color)
}
