extern crate alloc;

use alloc::vec::Vec;
use num::{One, Zero};
use num_complex::Complex64;
use core::ops::{Mul, MulAssign};

pub trait Inverse: Copy + Zero + MulAssign + One + Mul<Output = Self> {
    fn inverse_or_zero(self) -> Self;

    fn batch_inverse_or_zero(batch: &[Self]) -> Vec<Self> {
        batch.iter().map(|&x| x.inverse_or_zero()).collect()
    }
}

impl Inverse for Complex64 {
    fn inverse_or_zero(self) -> Self {
        if self.is_zero() {
            Complex64::zero()
        } else {
            let norm_sq = self.norm_sqr();
            Complex64::new(self.re / norm_sq, -self.im / norm_sq)
        }
    }
}

impl Inverse for f64 {
    fn inverse_or_zero(self) -> Self {
        if self.is_zero() {
            0.0
        } else {
            1.0 / self
        }
    }
}
