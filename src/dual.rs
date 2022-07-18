use std::{ops::{Neg, Add, Sub, Mul, Div, Rem}, cmp::PartialOrd};
use num_traits::{real::Real, Zero, One, Num, NumCast, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dual<T: Real> {
	pub r: T,
	pub d: T,
}

impl<T: Real> Dual<T> {
	pub fn new(r: T) -> Self {
		Self {r, d: T::zero()}
	}
}

impl<T: Real> Neg for Dual<T> {
	type Output = Self;
	fn neg(self) -> Self { Self {r: -self.r, d: -self.d} }
}

impl<T: Real> PartialOrd for Dual<T> {
	fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
		// Might not make sense
		self.r.partial_cmp(&rhs.r)
	}
}

impl<T: Real> ToPrimitive for Dual<T> {
	fn to_i64(&self) -> Option<i64> { self.r.to_i64() }
	fn to_u64(&self) -> Option<u64> { self.r.to_u64() }
}

impl<T: Real> NumCast for Dual<T> {
	fn from<TFrom: ToPrimitive>(x: TFrom) -> Option<Self> {
		T::from(x).map(|x| Self::new(x))
	}
}

impl<T: Real> Add for Dual<T> {
	type Output = Self;
	fn add(self, rhs: Self) -> Self {
		Self { r: self.r + rhs.r, d: self.d + rhs.d }
	}
}

impl<T: Real> Sub for Dual<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { r: self.r - rhs.r, d: self.d - rhs.d }
    }
}

impl<T: Real> Mul for Dual<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self { r: self.r * rhs.r, d: self.r * rhs.d + self.d * rhs.r }
    }
}

impl<T: Real> Div for Dual<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self { r: self.r / rhs.r, d: (self.d * rhs.r - self.r * rhs.d) / (rhs.r * rhs.r) }
    }
}

impl<T: Real> Rem for Dual<T> {
    type Output = Self;
    fn rem(self, _: Self) -> Self {
        Self::zero()
    }
}

impl<T: Real> Zero for Dual<T> {
	fn zero() -> Self {
		Self::new(T::zero())
	}

	fn is_zero(&self) -> bool { self.r.is_zero() }
}

impl<T: Real> One for Dual<T> {
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T: Real> Num for Dual<T> {
	type FromStrRadixErr = String;
	fn from_str_radix(_: &str, _: u32) -> Result<Self, String> { todo!() }
}

impl<T: Real> Real for Dual<T> {
	fn min_value() -> Self { todo!() }
	fn min_positive_value() -> Self { todo!() }
	fn epsilon() -> Self { todo!() }
	fn max_value() -> Self { todo!() }
	fn floor(self) -> Self { todo!() }
	fn ceil(self) -> Self { todo!() }
	fn round(self) -> Self { todo!() }
	fn trunc(self) -> Self { todo!() }
	fn fract(self) -> Self { todo!() }
	fn abs(self) -> Self { todo!() }
	fn signum(self) -> Self { todo!() }
	fn is_sign_positive(self) -> bool { todo!() }
	fn is_sign_negative(self) -> bool { todo!() }
	fn mul_add(self, _: Self, _: Self) -> Self { todo!() }
	fn recip(self) -> Self { todo!() }
	fn powi(self, _: i32) -> Self { todo!() }
	fn powf(self, _: Self) -> Self { todo!() }
	fn sqrt(self) -> Self { todo!() }
	fn exp(self) -> Self { todo!() }
	fn exp2(self) -> Self { todo!() }
	fn ln(self) -> Self { todo!() }
	fn log(self, _: Self) -> Self { todo!() }
	fn log2(self) -> Self { todo!() }
	fn log10(self) -> Self { todo!() }
	fn to_degrees(self) -> Self { todo!() }
	fn to_radians(self) -> Self { todo!() }
	fn max(self, _: Self) -> Self { todo!() }
	fn min(self, _: Self) -> Self { todo!() }
	fn abs_sub(self, _: Self) -> Self { todo!() }
	fn cbrt(self) -> Self { todo!() }
	fn hypot(self, _: Self) -> Self { todo!() }
	fn sin(self) -> Self { todo!() }
	fn cos(self) -> Self { todo!() }
	fn tan(self) -> Self { todo!() }
	fn asin(self) -> Self { todo!() }
	fn acos(self) -> Self { todo!() }
	fn atan(self) -> Self { todo!() }
	fn atan2(self, _: Self) -> Self { todo!() }
	fn sin_cos(self) -> (Self, Self) { todo!() }
	fn exp_m1(self) -> Self { todo!() }
	fn ln_1p(self) -> Self { todo!() }
	fn sinh(self) -> Self { todo!() }
	fn cosh(self) -> Self { todo!() }
	fn tanh(self) -> Self { todo!() }
	fn asinh(self) -> Self { todo!() }
	fn acosh(self) -> Self { todo!() }
	fn atanh(self) -> Self { todo!() }
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn new() {
        let x = Dual::new(3.14);
        println!("{:?}", x);
    }

	#[test]
	fn arithmetic() {
		let a = Dual {r: 2.0, d: 1.0};
		let b = Dual {r: 0.5, d: 2.0};

		let c = a / b;
		println!("{:?}", c);
	}	
}

