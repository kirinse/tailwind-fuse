use crate::utils::parse_fraction;
use std::str::FromStr;

use super::*;

impl<'a> TailwindArbitrary<'a> {
    #[inline]
    pub fn is_some(&self) -> bool {
        !self.inner.is_empty()
    }
    #[inline]
    pub fn is_none(&self) -> bool {
        self.inner.is_empty()
    }
    #[inline]
    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }
    #[inline]
    pub fn as_integer(&self) -> Result<i32> {
        Ok(i32::from_str(&self.inner)?)
    }
    #[inline]
    pub fn as_float(&self) -> Result<f32> {
        Ok(f32::from_str(&self.inner)?)
    }
    #[inline]
    pub fn as_fraction(&self) -> Result<(usize, usize)> {
        parse_fraction(&self.inner)
    }

    #[inline]
    pub fn as_length(&self) -> Result<LengthUnit> {
        LengthUnit::parse_length(&self.inner)
    }
    #[inline]
    pub fn as_length_or_fraction(&self) -> Result<LengthUnit> {
        LengthUnit::parse_length(&self.inner).or_else(|_| LengthUnit::parse_fraction(&self.inner))
    }
    #[inline]
    pub fn as_angle(&self) -> Result<LengthUnit> {
        LengthUnit::parse_angle(&self.inner)
    }
}
