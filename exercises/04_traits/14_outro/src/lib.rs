#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    pub fn new(value: u16) -> Self {
        SaturatingU16 { value }
    }
    
    pub fn get(&self) -> u16 {
        self.value
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value.into(), 
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> Self {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 { value: (*value).into() }
    }
}

impl From<SaturatingU16> for u16 {
    fn from(value: SaturatingU16) -> u16 {
        value.value
    }
}

impl std::ops::Add for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl std::ops::Add<u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs),
        }
    }
}

impl std::ops::Add<&u16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &u16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(*rhs),
        }
    }
}

impl std::ops::Add<&SaturatingU16> for SaturatingU16 {
    type Output = Self;

    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        SaturatingU16 {
            value: self.value.saturating_add(rhs.value),
        }
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        self.value == *other
    }
}