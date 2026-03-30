//! Absent dimensions.

use zahl::Z;

use crate::api::{Dimension, Unit};

//
//  Dimension
//

/// Zero dimension, when the dimension has not been decided yet.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ZeroDimension;

impl Dimension for ZeroDimension {
    type Exponent = Z<0>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        ""
    }
}

impl core::ops::Neg for ZeroDimension {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl core::ops::AddAssign for ZeroDimension {
    fn add_assign(&mut self, _other: Self) {
        //  Empty.
    }
}

impl core::ops::Add for ZeroDimension {
    type Output = Self;

    fn add(self, _other: Self) -> Self::Output {
        self
    }
}

impl core::ops::SubAssign for ZeroDimension {
    fn sub_assign(&mut self, _other: Self) {
        //  Empty.
    }
}

impl core::ops::Sub for ZeroDimension {
    type Output = Self;

    fn sub(self, _other: Self) -> Self::Output {
        self
    }
}

impl core::ops::Mul for ZeroDimension {
    type Output = Self;

    fn mul(self, _other: Self) -> Self::Output {
        self
    }
}

impl core::ops::Div for ZeroDimension {
    type Output = Self;

    fn div(self, _other: Self) -> Self::Output {
        self
    }
}

//
//  Unit
//

/// An absence of known unit.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Zero(ZeroDimension);

impl Zero {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(ZeroDimension)
    }
}

impl Unit for Zero {
    type Dimension = ZeroDimension;
    type Exponent = Z<0>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        ""
    }
}

impl core::ops::Neg for Zero {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self
    }
}

impl core::ops::AddAssign for Zero {
    fn add_assign(&mut self, _other: Self) {
        //  Empty.
    }
}

impl core::ops::Add for Zero {
    type Output = Self;

    fn add(self, _other: Self) -> Self::Output {
        self
    }
}

impl core::ops::SubAssign for Zero {
    fn sub_assign(&mut self, _other: Self) {
        //  Empty.
    }
}

impl core::ops::Sub for Zero {
    type Output = Self;

    fn sub(self, _other: Self) -> Self::Output {
        self
    }
}

impl core::ops::Mul for Zero {
    type Output = Self;

    fn mul(self, _other: Self) -> Self::Output {
        self
    }
}

impl core::ops::Div for Zero {
    type Output = Self;

    fn div(self, _other: Self) -> Self::Output {
        self
    }
}
