//! A collection of scales.

use core::num::NonZeroI64;

//
//  Non-transformations.
//

/// A scale of One. No matter what it multiplies, it changes nothing.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct One;

impl<Q> core::ops::Mul<Q> for One {
    type Output = Q;

    fn mul(self, other: Q) -> Self::Output {
        other
    }
}

//
//  Basic transformations.
//

macro_rules! impl_from_assign {
    //  Implements Add<$b> for $a and Add<$a> for $b from AddAssign<$b> for $a.
    ($a:ident + $b:ident) => {
        impl core::ops::Add<$b> for $a {
            type Output = $a;

            fn add(mut self, other: $b) -> Self::Output {
                self += other;

                self
            }
        }

        impl core::ops::Add<$a> for $b {
            type Output = $a;

            fn add(self, other: $a) -> Self::Output {
                other + self
            }
        }
    };
    //  Implements Sub<$b> for $a SubAssign<$b> for $a.
    ($a:ident - $b:ident) => {
        impl core::ops::Sub<$b> for $a {
            type Output = $a;

            fn sub(mut self, other: $b) -> Self::Output {
                self -= other;

                self
            }
        }
    };
    //  Implements Mul<$b> for $a and Mul<$a> for $b from MulAssign<$b> for $a.
    ($a:ident * $b:ident) => {
        impl core::ops::Mul<$b> for $a {
            type Output = $a;

            fn mul(mut self, other: $b) -> Self::Output {
                self *= other;

                self
            }
        }

        impl core::ops::Mul<$a> for $b {
            type Output = $a;

            fn mul(self, other: $a) -> Self::Output {
                other * self
            }
        }
    };
    //  Implements Div<$b> for $a from DivAssign<$b> for $a.
    ($a:ident / $b:ident) => {
        impl core::ops::Div<$b> for $a {
            type Output = $a;

            fn div(mut self, other: $b) -> Self::Output {
                self /= other;

                self
            }
        }
    };
}

/// An integral.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Integral(NonZeroI64);

impl Integral {
    /// Constructs a new integral.
    pub const fn new(n: NonZeroI64) -> Self {
        Self(n)
    }

    /// Returns as ratio.
    pub const fn into_ratio(self) -> IntegralRatio {
        const ONE: NonZeroI64 = NonZeroI64::new(1).unwrap();

        IntegralRatio::new(self.0, ONE)
    }

    /// Returns the integral as a f32.
    pub const fn as_f32(&self) -> f32 {
        self.0.get() as f32
    }

    /// Returns the integral as a f64.
    pub const fn as_f64(&self) -> f64 {
        self.0.get() as f64
    }
}

impl core::ops::Neg for Integral {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl core::ops::AddAssign<Integral> for i64 {
    fn add_assign(&mut self, other: Integral) {
        *self += other.0.get();
    }
}

impl core::ops::SubAssign<Integral> for i64 {
    fn sub_assign(&mut self, other: Integral) {
        *self -= other.0.get();
    }
}

impl core::ops::MulAssign<Integral> for i64 {
    fn mul_assign(&mut self, other: Integral) {
        *self *= other.0.get();
    }
}

impl core::ops::DivAssign<Integral> for i64 {
    fn div_assign(&mut self, other: Integral) {
        *self /= other.0.get();
    }
}

impl_from_assign!(i64 + Integral);
impl_from_assign!(i64 - Integral);
impl_from_assign!(i64 * Integral);
impl_from_assign!(i64 / Integral);

impl core::ops::AddAssign<Integral> for f32 {
    fn add_assign(&mut self, other: Integral) {
        *self += other.as_f32();
    }
}

impl core::ops::SubAssign<Integral> for f32 {
    fn sub_assign(&mut self, other: Integral) {
        *self -= other.as_f32();
    }
}

impl core::ops::MulAssign<Integral> for f32 {
    fn mul_assign(&mut self, other: Integral) {
        *self *= other.as_f32();
    }
}

impl core::ops::DivAssign<Integral> for f32 {
    fn div_assign(&mut self, other: Integral) {
        *self /= other.as_f32();
    }
}

impl_from_assign!(f32 + Integral);
impl_from_assign!(f32 - Integral);
impl_from_assign!(f32 * Integral);
impl_from_assign!(f32 / Integral);

impl core::ops::AddAssign<Integral> for f64 {
    fn add_assign(&mut self, other: Integral) {
        *self += other.as_f64();
    }
}

impl core::ops::SubAssign<Integral> for f64 {
    fn sub_assign(&mut self, other: Integral) {
        *self -= other.as_f64();
    }
}

impl core::ops::MulAssign<Integral> for f64 {
    fn mul_assign(&mut self, other: Integral) {
        *self *= other.as_f64();
    }
}

impl core::ops::DivAssign<Integral> for f64 {
    fn div_assign(&mut self, other: Integral) {
        *self /= other.as_f64();
    }
}

impl_from_assign!(f64 + Integral);
impl_from_assign!(f64 - Integral);
impl_from_assign!(f64 * Integral);
impl_from_assign!(f64 / Integral);

/// A ratio.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct IntegralRatio {
    num: NonZeroI64,
    den: NonZeroI64,
}

impl IntegralRatio {
    /// Constructs a new ratio of value num / den.
    pub const fn new(num: NonZeroI64, den: NonZeroI64) -> Self {
        Self { num, den }
    }

    /// Returns the inverse of the ratio.
    pub const fn invert(self) -> Self {
        Self {
            num: self.den,
            den: self.num,
        }
    }

    /// Returns the ratio as a f32.
    pub const fn as_f32(&self) -> f32 {
        self.num.get() as f32 / (self.den.get() as f32)
    }

    /// Returns the ratio as a f64.
    pub const fn as_f64(&self) -> f64 {
        self.num.get() as f64 / (self.den.get() as f64)
    }
}

impl core::ops::Neg for IntegralRatio {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.num = -self.num;

        self
    }
}

impl core::ops::MulAssign<IntegralRatio> for i64 {
    fn mul_assign(&mut self, other: IntegralRatio) {
        *self *= other.num.get();
        *self /= other.den.get();
    }
}

impl core::ops::DivAssign<IntegralRatio> for i64 {
    fn div_assign(&mut self, other: IntegralRatio) {
        *self *= other.den.get();
        *self /= other.num.get();
    }
}

impl_from_assign!(i64 * IntegralRatio);
impl_from_assign!(i64 / IntegralRatio);

impl core::ops::AddAssign<IntegralRatio> for f32 {
    fn add_assign(&mut self, other: IntegralRatio) {
        *self += other.as_f32();
    }
}

impl core::ops::SubAssign<IntegralRatio> for f32 {
    fn sub_assign(&mut self, other: IntegralRatio) {
        *self -= other.as_f32();
    }
}

impl core::ops::MulAssign<IntegralRatio> for f32 {
    fn mul_assign(&mut self, other: IntegralRatio) {
        *self *= other.as_f32();
    }
}

impl core::ops::DivAssign<IntegralRatio> for f32 {
    fn div_assign(&mut self, other: IntegralRatio) {
        *self /= other.as_f32();
    }
}

impl_from_assign!(f32 + IntegralRatio);
impl_from_assign!(f32 - IntegralRatio);
impl_from_assign!(f32 * IntegralRatio);
impl_from_assign!(f32 / IntegralRatio);

impl core::ops::AddAssign<IntegralRatio> for f64 {
    fn add_assign(&mut self, other: IntegralRatio) {
        *self += other.as_f64();
    }
}

impl core::ops::SubAssign<IntegralRatio> for f64 {
    fn sub_assign(&mut self, other: IntegralRatio) {
        *self -= other.as_f64();
    }
}

impl core::ops::MulAssign<IntegralRatio> for f64 {
    fn mul_assign(&mut self, other: IntegralRatio) {
        *self *= other.as_f64();
    }
}

impl core::ops::DivAssign<IntegralRatio> for f64 {
    fn div_assign(&mut self, other: IntegralRatio) {
        *self /= other.as_f64();
    }
}

impl_from_assign!(f64 + IntegralRatio);
impl_from_assign!(f64 - IntegralRatio);
impl_from_assign!(f64 * IntegralRatio);
impl_from_assign!(f64 / IntegralRatio);
