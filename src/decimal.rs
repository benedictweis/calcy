use num::FromPrimitive;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Decimal {
    value: u128,
    scale: u8,
    negative: bool,
}

#[derive(Debug, PartialEq)]
pub struct ParseDecimalError {}

impl Decimal {
    pub fn new(value: u128, scale: u8, negative: bool) -> Self {
        Self { value, scale, negative }
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pre = self.value / 10_u128.pow(self.scale as u32);
        if self.scale == 0 {
            write!(f, "{pre}")
        } else {
            let after = self.value % 10_u128.pow(self.scale as u32);
            write!(f, "{pre}.{after}")
        }
    }
}

impl FromStr for Decimal {
    type Err = ParseDecimalError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let scale = match s.find('.') {
            Some(i) => {
                u8::from_usize(s.len()).ok_or(|| ParseDecimalError {}).map_err(|_| ParseDecimalError {})? - u8::from_usize(i).ok_or(|| ParseDecimalError {}).map_err(|_| ParseDecimalError {})? - 1
            }
            None => 0,
        };

        let negative = s.starts_with('-');

        let value_str = s.replace(['.', '-'], "");
        let value = value_str.parse::<u128>().map_err(|_| ParseDecimalError {})?;

        Ok(Decimal { value, scale, negative })
    }
}
