//! Collection of dimensions to use across systems.
//!
//! Using the same dimension across various systems allows for better interoperability between systems, by making it
//! safer to convert from one system to another without accidentally swapping dimensions.

use zahl::Z;

use crate::{api::Dimension, kit::zero::ZeroDimension};

//
//  A collection of dimension traits.
//

/// A dimension for angles.
///
/// In most systems, angles have no dimension. This is mathematically and physically accurate. Unfortunately, it is also
/// impractical, as angles _still_ have units (degrees vs radians), and references (radians vs steradians).
pub trait AngleDimension: Dimension {}

/// Length dimension.
pub trait LengthDimension: Dimension {}

/// Mass dimension.
pub trait MassDimension: Dimension {}

/// Time dimension.
pub trait TimeDimension: Dimension {}

/// Electric current dimension.
pub trait CurrentDimension: Dimension {}

/// Thermodynamic temperature dimension.
pub trait TemperatureDimension: Dimension {}

/// Luminuous intensity dimension.
pub trait LuminousIntensityDimension: Dimension {}

/// Amount of substance dimension.
pub trait AmountOfSubstanceDimension: Dimension {}

//
//  A neutral dimension type.
//

impl AngleDimension for ZeroDimension {}
impl LengthDimension for ZeroDimension {}
impl MassDimension for ZeroDimension {}
impl TimeDimension for ZeroDimension {}
impl CurrentDimension for ZeroDimension {}
impl TemperatureDimension for ZeroDimension {}
impl LuminousIntensityDimension for ZeroDimension {}
impl AmountOfSubstanceDimension for ZeroDimension {}

//
//  A collection of dimension types.
//

macro_rules! impl_arithmetic_dimension {
    ($name:ident) => {
        impl<E> core::ops::Neg for $name<E> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::AddAssign for $name<E> {
            fn add_assign(&mut self, _other: Self) {
                //  Empty.
            }
        }

        impl<E> core::ops::Add for $name<E> {
            type Output = Self;

            fn add(self, _other: Self) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::SubAssign for $name<E> {
            fn sub_assign(&mut self, _other: Self) {
                //  Empty.
            }
        }

        impl<E> core::ops::Sub for $name<E> {
            type Output = Self;

            fn sub(self, _other: Self) -> Self::Output {
                self
            }
        }

        impl<E, OE> core::ops::Mul<$name<OE>> for $name<E>
        where
            E: core::ops::Add<OE>,
        {
            type Output = $name<E::Output>;

            fn mul(self, other: $name<OE>) -> Self::Output {
                //  Exponent arithmetic.
                #[allow(clippy::suspicious_arithmetic_impl)]
                $name(self.0 + other.0)
            }
        }

        impl<E> core::ops::Mul<ZeroDimension> for $name<E> {
            type Output = $name<E>;

            fn mul(self, _other: ZeroDimension) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::Mul<$name<E>> for ZeroDimension {
            type Output = $name<E>;

            fn mul(self, other: $name<E>) -> Self::Output {
                other
            }
        }

        impl<E, OE> core::ops::Div<$name<OE>> for $name<E>
        where
            E: core::ops::Sub<OE>,
        {
            type Output = $name<E::Output>;

            fn div(self, other: $name<OE>) -> Self::Output {
                //  Exponent arithmetic.
                #[allow(clippy::suspicious_arithmetic_impl)]
                $name(self.0 - other.0)
            }
        }

        impl<E> core::ops::Div<ZeroDimension> for $name<E> {
            type Output = $name<E>;

            fn div(self, _other: ZeroDimension) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::Div<$name<E>> for ZeroDimension
        where
            E: core::ops::Neg,
        {
            type Output = $name<E::Output>;

            fn div(self, other: $name<E>) -> Self::Output {
                $name(-other.0)
            }
        }
    };
}

/// Shortcut to reference an Angle dimension of power N.
pub type Angle<const N: i32 = 1> = AngleT<Z<N>>;

/// The underlying angle dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AngleT<E>(E);

impl<const N: i32> AngleT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for AngleT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        ""
    }
}

impl<const N: i32> AngleDimension for AngleT<Z<N>> {}

impl_arithmetic_dimension!(AngleT);

/// Shortcut to reference a Length dimension of power N.
pub type Length<const N: i32 = 1> = LengthT<Z<N>>;

/// The underlying Length dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LengthT<E>(E);

impl<const N: i32> LengthT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for LengthT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "L"
    }
}

impl<const N: i32> LengthDimension for LengthT<Z<N>> {}

impl_arithmetic_dimension!(LengthT);

/// Shortcut to reference a mass dimension of power N.
pub type Mass<const N: i32 = 1> = MassT<Z<N>>;

/// The underlying mass dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MassT<E>(E);

impl<const N: i32> MassT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for MassT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "M"
    }
}

impl<const N: i32> MassDimension for MassT<Z<N>> {}

impl_arithmetic_dimension!(MassT);

/// Shortcut to reference a time dimension of power N.
pub type Time<const N: i32 = 1> = TimeT<Z<N>>;

/// The underlying time dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeT<E>(E);

impl<const N: i32> TimeT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for TimeT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "M"
    }
}

impl<const N: i32> TimeDimension for TimeT<Z<N>> {}

impl_arithmetic_dimension!(TimeT);

/// Shortcut to reference a current dimension of power N.
pub type Current<const N: i32 = 1> = CurrentT<Z<N>>;

/// The underlying current dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CurrentT<E>(E);

impl<const N: i32> CurrentT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for CurrentT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "I"
    }
}

impl<const N: i32> CurrentDimension for CurrentT<Z<N>> {}

impl_arithmetic_dimension!(CurrentT);

/// Shortcut to reference a temperature dimension of power N.
pub type Temperature<const N: i32 = 1> = TemperatureT<Z<N>>;

/// The underlying temperature dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TemperatureT<E>(E);

impl<const N: i32> TemperatureT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for TemperatureT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "Θ"
    }
}

impl<const N: i32> TemperatureDimension for TemperatureT<Z<N>> {}

impl_arithmetic_dimension!(TemperatureT);

/// Shortcut to reference a luminous intensity dimension of power N.
pub type LuminousIntensity<const N: i32 = 1> = LuminousIntensityT<Z<N>>;

/// The underlying luminous intensity dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LuminousIntensityT<E>(E);

impl<const N: i32> LuminousIntensityT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for LuminousIntensityT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "J"
    }
}

impl<const N: i32> LuminousIntensityDimension for LuminousIntensityT<Z<N>> {}

impl_arithmetic_dimension!(LuminousIntensityT);

/// Shortcut to reference an amount of substance dimension of power N.
pub type AmountOfSubstance<const N: i32 = 1> = AmountOfSubstanceT<Z<N>>;

/// The underlying amount of substance dimension type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AmountOfSubstanceT<E>(E);

impl<const N: i32> AmountOfSubstanceT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(Z::new())
    }
}

impl<const N: i32> Dimension for AmountOfSubstanceT<Z<N>> {
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn exponent(&self) -> Self::Exponent {
        Z
    }

    fn symbol(&self) -> Self::Symbol {
        "N"
    }
}

impl<const N: i32> AmountOfSubstanceDimension for AmountOfSubstanceT<Z<N>> {}

impl_arithmetic_dimension!(AmountOfSubstanceT);
