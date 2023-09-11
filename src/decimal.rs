use num::traits::Pow;
use num::FromPrimitive;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Decimal {
    value: i128,
    scale: u8,
}

#[derive(Debug, PartialEq)]
pub struct ParseDecimalError {}

impl Decimal {
    pub fn new(value: i128, scale: u8) -> Self {
        Self { value, scale }
    }

    pub fn scale(&mut self, scale: u8) {
        let div = scale as i16 - self.scale as i16;
        self.scale = scale;
        if div == 0 {
            return;
        }
        if div > 0 {
            self.value *= 10_i128.pow(div.unsigned_abs() as u32);
        } else {
            self.value /= 10_i128.pow(div.unsigned_abs() as u32);
        }
    }

    pub fn pack(&mut self) {
        if self.value == 0 {
            self.scale(0);
            return;
        }
        let mut i: u8 = 1;
        while self.value % 10_i128.pow(i as u32) == 0 {
            i += 1;
        }
        let this = self.scale - (i - 1);
        self.scale(this)
    }
}

pub fn scale_smallest(a: &mut Decimal, b: &mut Decimal) {
    if a.scale == b.scale {
        return;
    }
    if a.scale < b.scale {
        a.scale(b.scale);
    } else {
        b.scale(a.scale);
    }
}

impl Display for Decimal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let pre = self.value / 10_i128.pow(self.scale as u32);
        if self.scale == 0 {
            write!(f, "{pre}")
        } else {
            let after = self.value % 10_i128.pow(self.scale as u32);
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

        let value_str = s.replace('.', "");
        let value = value_str.parse::<i128>().map_err(|_| ParseDecimalError {})?;

        Ok(Decimal::new(value, scale))
    }
}

impl Add for Decimal {
    type Output = Decimal;

    fn add(mut self, mut rhs: Self) -> Self::Output {
        scale_smallest(&mut self, &mut rhs);
        self.value += rhs.value;
        self.pack();
        self
    }
}

impl Sub for Decimal {
    type Output = Decimal;

    fn sub(mut self, mut rhs: Self) -> Self::Output {
        scale_smallest(&mut self, &mut rhs);
        self.value -= rhs.value;
        self.pack();
        self
    }
}

impl Mul for Decimal {
    type Output = Decimal;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for Decimal {
    type Output = Decimal;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Pow<Decimal> for Decimal {
    type Output = Decimal;

    fn pow(self, _rhs: Decimal) -> Self::Output {
        todo!()
    }
}
