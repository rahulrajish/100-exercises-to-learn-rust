// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.
use std::ops::Add;
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SaturatingU16 {
    pub value: u16,
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        Self { value }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other  
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        Self {
            value
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        Self {
            value: *value,
        }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> SaturatingU16 {
        Self {
            value: value as u16
        }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> SaturatingU16 {
        Self {
            value: *value as u16
        }
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(mut self, val: u16) -> SaturatingU16 {
        if val != u16::MAX {
            self.value += val; 
        } else {
            self.value = u16::MAX
        }
        self
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(mut self, val: &u16) -> SaturatingU16 {
        if *val != u16::MAX {
            self.value += *val; 
        } else {
            self.value = u16::MAX
        }
        self
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(mut self, val: SaturatingU16) -> SaturatingU16 {
        if val.value != u16::MAX {
            self.value += val.value; 
        } else {
            self.value = u16::MAX
        }
        self
    }
}




