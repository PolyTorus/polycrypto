use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use rand_distr::{
    num_traits::{One, Zero},
    Distribution,
    Standard,
};
use super::cyclotomic_fourier::CyclotomicFourier;
use super::inverse::Inverse;

const Q: u32 = 1073754113u32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct U32Field(pub u32);

impl U32Field {
    pub const fn new(value: i32) -> Self {
        let gtz_bool: bool = value >= 0;
        let gtz_int: i32 = gtz_bool as i32;
        let gtz_sign: i32 = gtz_int - ((!gtz_int) as i32);
        let reduced = gtz_sign * ((gtz_sign * value) % (Q as i32));
        let canonical_representative = (reduced + (Q as i32) * (1 - gtz_int)) as u32;

        U32Field(canonical_representative)
    }

    pub const fn value(&self) -> i32 {
        self.0 as i32
    }

    pub fn balanced_value(&self) -> i32 {
        let value = self.value();
        let g = (value > ((Q as i32) / 2)) as i32;
        value - (Q as i32) * g
    }

    pub const fn multiply(&self, other: Self) -> Self {
        U32Field((((self.0 as u64) * (other.0 as u64)) % (Q as u64)) as u32)
    }

    pub fn from_usize(value: usize) -> Self {
        U32Field::new(value as i32)
    }
}
