#[derive(Debug, Clone, PartialEq)]
pub enum BObject {
    Integer(BInteger),
    Float(BFloat),
    Bool(BBool),
    String(BString),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BString {
    inner: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BIntegerVariant {
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BInteger {
    pub inner: BIntegerVariant,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BFloat {
    pub inner: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BBool {
    pub inner: bool,
}

// Impl
impl BInteger {
    pub fn new(intv: BIntegerVariant) -> Self {
        Self { inner: intv }
    }
}

impl BFloat {
    pub fn new(num: f64) -> Self {
        Self { inner: num }
    }
}
