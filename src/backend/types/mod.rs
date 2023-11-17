use std::num::IntErrorKind;

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
enum ParseIntStage {
    Int8,
    Int16,
    Int32,
    Int64,
    Int128,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
}

impl ParseIntStage {}

macro_rules! b_int {
    ($variant:ident, $val:expr) => {
        BInteger::new(BIntegerVariant::$variant($val))
    };
}

// Impl
impl BInteger {
    pub fn new(intv: BIntegerVariant) -> Self {
        Self { inner: intv }
    }

    fn parse_lower(string: &str, stage: ParseIntStage) -> BInteger {
        use ParseIntStage as S;
        match stage {
            S::Int8 => match string.parse::<i8>() {
                Ok(num) => {
                    b_int!(Int8, num)
                }
                Err(_) => BInteger::parse_lower(string, S::Int16),
            },
            S::Int16 => match string.parse::<i16>() {
                Ok(num) => {
                    b_int!(Int16, num)
                }
                Err(_) => BInteger::parse_lower(string, S::Int32),
            },
            S::Int32 => match string.parse::<i32>() {
                Ok(num) => {
                    b_int!(Int32, num)
                }
                Err(_) => BInteger::parse_lower(string, S::Int64),
            },
            S::Int64 => match string.parse::<i64>() {
                Ok(num) => {
                    b_int!(Int64, num)
                }
                Err(_) => BInteger::parse_lower(string, S::Int128),
            },
            S::Int128 => match string.parse::<i128>() {
                Ok(num) => {
                    b_int!(Int128, num)
                }
                Err(e) => panic!("Number {} is out of the bounds of type Int128!", e),
            },
            _ => unreachable!(),
        }
    }

    fn parse_higher(string: &str, stage: ParseIntStage) -> BInteger {
        use ParseIntStage as S;
        match stage {
            S::Uint16 => match string.parse::<u16>() {
                Ok(num) => {
                    b_int!(Uint16, num)
                }
                Err(_) => BInteger::parse_higher(string, S::Uint32),
            },
            S::Uint32 => match string.parse::<u32>() {
                Ok(num) => {
                    b_int!(Uint32, num)
                }
                Err(_) => BInteger::parse_higher(string, S::Uint64),
            },
            S::Uint64 => match string.parse::<u64>() {
                Ok(num) => {
                    b_int!(Uint64, num)
                }
                Err(_) => BInteger::parse_higher(string, S::Uint128),
            },
            S::Uint128 => match string.parse::<u128>() {
                Ok(num) => {
                    b_int!(Uint128, num)
                }
                Err(e) => panic!("Number {} is out of the bounds of type Int128!", e),
            },
            _ => unreachable!(),
        }
    }

    pub fn parse_from_string(string: &str) -> Self {
        match string.parse::<u8>() {
            Ok(num) => BInteger::new(BIntegerVariant::Uint8(num)),
            Err(e) => match e.kind() {
                IntErrorKind::NegOverflow => BInteger::parse_lower(string, ParseIntStage::Int8),
                IntErrorKind::PosOverflow => BInteger::parse_higher(string, ParseIntStage::Uint16),
                _ => panic!("Unexpected error whilst parsing number: {}", e),
            },
        }
    }
}

impl BFloat {
    pub fn new(num: f64) -> Self {
        Self { inner: num }
    }
}
