//! Compile-time exponents.

use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use zahl::{DivOr, Pow as Power, Root, RootOr, Z};

/// Compile 2.pow(N).
pub type Two<const N: i32> = Pow<2, Z<N>>;

/// Compile-time 10.pow(N).
pub type Ten<const N: i32> = Pow<10, Z<N>>;

/// Compile-time I.pow(Exp).
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pow<const I: i64, Exp>(Exp);

impl<const I: i64, const N: i32> Pow<I, Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

//  FIXME: add way to get a ratio out of this.

//
//  Arithmetic: addition/subtraction.
//

impl<const I: i64, E> AddAssign<Pow<I, E>> for Pow<I, E> {
    fn add_assign(&mut self, _other: Self) {
        //  Empty.
    }
}

impl<const I: i64, E> Add<Pow<I, E>> for Pow<I, E> {
    type Output = Self;

    fn add(self, _other: Self) -> Self::Output {
        self
    }
}

impl<const I: i64, E> Neg for Pow<I, E> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl<const I: i64, E> SubAssign<Pow<I, E>> for Pow<I, E> {
    fn sub_assign(&mut self, _other: Self) {
        //  Empty.
    }
}

impl<const I: i64, E> Sub<Pow<I, E>> for Pow<I, E> {
    type Output = Self;

    fn sub(self, _other: Self) -> Self::Output {
        self
    }
}

//
//  Arithmetic: multiplication/division.
//

impl<const I: i64, E, OE> Div<Pow<I, OE>> for Pow<I, E>
where
    E: Sub<OE>,
{
    type Output = Pow<I, E::Output>;

    fn div(self, other: Pow<I, OE>) -> Self::Output {
        //  Exponent arithmetic.
        #![allow(clippy::suspicious_arithmetic_impl)]

        Pow(self.0 - other.0)
    }
}

impl<const I: i64, E, OE> Mul<Pow<I, OE>> for Pow<I, E>
where
    E: Add<OE>,
{
    type Output = Pow<I, E::Output>;

    fn mul(self, other: Pow<I, OE>) -> Self::Output {
        //  Exponent arithmetic.
        #![allow(clippy::suspicious_arithmetic_impl)]

        Pow(self.0 + other.0)
    }
}

//
//  Arithmetic: power/root.
//

impl<const I: i64, E, OE> Power<OE> for Pow<I, E>
where
    E: Mul<OE>,
{
    type Output = Pow<I, E::Output>;

    fn pow(self, other: OE) -> Self::Output {
        Pow(self.0 * other)
    }
}

impl<const I: i64, E, OE> Root<OE> for Pow<I, E>
where
    E: Div<OE>,
{
    type Output = Pow<I, E::Output>;

    fn root(self, other: OE) -> Self::Output {
        Pow(self.0 / other)
    }
}

impl<const I: i64, E, OE, F> RootOr<OE, F> for Pow<I, E>
where
    E: DivOr<OE, F>,
{
    type Output = Pow<I, E::Output>;

    fn root_or(self, other: OE, fallback: F) -> Self::Output {
        Pow(self.0.div_or(other, fallback))
    }
}
