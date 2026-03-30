//! A scalar, dimensionless, value.
//!
//! Unfortunately, it is not possible to implement both `q * 3` and `q * q`, as the former requires a generic
//! implementation which clashes with the latter.
//!
//! The `Scalar` struct is a simple wrapper around a scalar value, for which operations can be implemented without
//! running into conflicts.

use core::{fmt, str};

/// A scalar value.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scalar<V>(V);

//
//  Construction/Destruction.
//

impl<V> Scalar<V> {
    /// Constructs a new quantity from a value.
    pub const fn new(value: V) -> Self {
        Self(value)
    }

    /// Returns the value.
    pub fn into_value(self) -> V {
        self.0
    }
}

//
//  String conversions
//

impl<V> fmt::Display for Scalar<V>
where
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl<V> str::FromStr for Scalar<V>
where
    V: str::FromStr,
{
    type Err = V::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Scalar::new)
    }
}
