use num_traits::{real::Real, Num, NumCast, One, ToPrimitive, Zero};
use std::{
    cmp::PartialOrd,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dual<T: Real> {
    pub r: T,
    pub d: T,
}

impl<T: Real> Dual<T> {
    pub fn new(r: T) -> Self {
        Self { r, d: T::zero() }
    }
}

impl<T: Real> Neg for Dual<T> {
    type Output = Self;
    fn neg(self) -> Self {
        Self {
            r: -self.r,
            d: -self.d,
        }
    }
}

impl<T: Real> PartialOrd for Dual<T> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        // Might not make sense
        self.r.partial_cmp(&rhs.r)
    }
}

impl<T: Real> ToPrimitive for Dual<T> {
    fn to_i64(&self) -> Option<i64> {
        self.r.to_i64()
    }
    fn to_u64(&self) -> Option<u64> {
        self.r.to_u64()
    }
}

impl<T: Real> NumCast for Dual<T> {
    fn from<TFrom: ToPrimitive>(x: TFrom) -> Option<Self> {
        T::from(x).map(|x| Self::new(x))
    }
}

impl<T: Real> Add for Dual<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            r: self.r + rhs.r,
            d: self.d + rhs.d,
        }
    }
}

impl<T: Real> Sub for Dual<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            r: self.r - rhs.r,
            d: self.d - rhs.d,
        }
    }
}

impl<T: Real> Mul for Dual<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self {
            r: self.r * rhs.r,
            d: self.r * rhs.d + self.d * rhs.r,
        }
    }
}

impl<T: Real> Div for Dual<T> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        Self {
            r: self.r / rhs.r,
            d: (self.d * rhs.r - self.r * rhs.d) / (rhs.r * rhs.r),
        }
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

    fn is_zero(&self) -> bool {
        self.r.is_zero()
    }
}

impl<T: Real> One for Dual<T> {
    fn one() -> Self {
        Self::new(T::one())
    }
}

impl<T: Real> Num for Dual<T> {
    type FromStrRadixErr = String;
    fn from_str_radix(_: &str, _: u32) -> Result<Self, String> {
        todo!()
    }
}

impl<T: Real> Real for Dual<T> {
    fn min_value() -> Self {
        Self::new(T::min_value())
    }
    fn min_positive_value() -> Self {
        Self::new(T::min_positive_value())
    }
    fn epsilon() -> Self {
        Self::new(T::epsilon())
    }
    fn max_value() -> Self {
        Self::new(T::max_value())
    }
    fn floor(self) -> Self {
        unimplemented!()
    }
    fn ceil(self) -> Self {
        unimplemented!()
    }
    fn round(self) -> Self {
        unimplemented!()
    }
    fn trunc(self) -> Self {
        unimplemented!()
    }
    fn fract(self) -> Self {
        unimplemented!()
    }
    fn abs(self) -> Self {
        self * self.signum()
    }
    fn signum(self) -> Self {
        use std::cmp::Ordering;
        match self.partial_cmp(&Self::zero()) {
            None => Self::zero(),
            Some(Ordering::Equal) => Self::zero(),
            Some(Ordering::Greater) => Self::one(),
            Some(Ordering::Less) => -Self::one(),
        }
    }
    fn is_sign_positive(self) -> bool {
        self.signum() > Self::zero()
    }
    fn is_sign_negative(self) -> bool {
        self.signum() < Self::zero()
    }
    fn mul_add(self, x: Self, b: Self) -> Self {
        self * x + b
    }
    fn recip(self) -> Self {
        Self::one() / self
    }
    fn powi(mut self, mut e: i32) -> Self {
        if e < 0 {
            self.recip().powi(-e)
        } else {
            let mut acc = Self::one();
            while e > 0 {
                if e & 1 == 1 {
                    acc = acc * self;
                }
                self = self * self;
				e /= 2;
            }
            acc
        }
    }
    fn powf(self, e: Self) -> Self {
        (e * self.ln()).exp()
    }
    fn sqrt(self) -> Self {
        self.powf(<Self as NumCast>::from(0.5).unwrap())
    }
    fn exp(self) -> Self {
        let r = self.r.exp();
        Self { r, d: r * self.d }
    }
    fn exp2(self) -> Self {
        <Self as NumCast>::from(2).unwrap().powf(self)
    }
    fn ln(self) -> Self {
        Self {
            r: self.r.ln(),
            d: self.d / self.r,
        }
    }
    fn log(self, b: Self) -> Self {
        self.ln() / b.ln()
    }
    fn log2(self) -> Self {
        self.log(<Self as NumCast>::from(2).unwrap())
    }
    fn log10(self) -> Self {
        self.log(<Self as NumCast>::from(10).unwrap())
    }
    fn to_degrees(self) -> Self {
        todo!()
    }
    fn to_radians(self) -> Self {
        todo!()
    }
    fn max(self, other: Self) -> Self {
        if self < other {
            other
        } else {
            self
        }
    }
    fn min(self, other: Self) -> Self {
        if self > other {
            other
        } else {
            self
        }
    }
    fn abs_sub(self, rhs: Self) -> Self {
        (self - rhs).abs()
    }
    fn cbrt(self) -> Self {
        self.powf(<Self as NumCast>::from(1.0 / 3.0).unwrap())
    }
    fn hypot(self, b: Self) -> Self {
        (self * self + b * b).sqrt()
    }
    fn sin(self) -> Self {
        let (r, d) = self.r.sin_cos();
        Self { r, d: self.d * d }
    }
    fn cos(self) -> Self {
        let (d, r) = self.r.sin_cos();
        Self { r, d: -self.d * d }
    }
    fn tan(self) -> Self {
        let (s, c) = self.sin_cos();
        s / c
    }
    fn asin(self) -> Self {
        todo!()
    }
    fn acos(self) -> Self {
        todo!()
    }
    fn atan(self) -> Self {
        todo!()
    }
    fn atan2(self, _: Self) -> Self {
        todo!()
    }
    fn sin_cos(self) -> (Self, Self) {
        let (s, c) = self.r.sin_cos();
        (
            Self {
                r: s,
                d: self.d * c,
            },
            Self {
                r: c,
                d: -self.d * s,
            },
        )
    }
    fn exp_m1(self) -> Self {
        todo!()
    }
    fn ln_1p(self) -> Self {
        todo!()
    }
    fn sinh(self) -> Self {
        todo!()
    }
    fn cosh(self) -> Self {
        todo!()
    }
    fn tanh(self) -> Self {
        todo!()
    }
    fn asinh(self) -> Self {
        todo!()
    }
    fn acosh(self) -> Self {
        todo!()
    }
    fn atanh(self) -> Self {
        todo!()
    }
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
        let a = Dual { r: 2.0, d: 1.0 };
        let b = Dual { r: 0.5, d: 2.0 };

        let c = a / b;
        println!("{:?}", c);
    }
}
