//! An exponent.
//!
//! The values of the exponents are lifted to types, so they can be manipulated at compile-time. Yet, they still need to
//! be lowered to values for various purposes.

use zahl::Z;

/// A trait which exponents must implement.
pub trait Exponent {
    /// The type of the runtime value of an exponent.
    ///
    /// For Z<N>, this is `i32`, whereas for a possible `Rational<N, D>` it could be a
    /// `struct Ratio { num: i32, den: u32}`.
    type Value;

    /// Returns the value associated with the exponent.
    fn value(&self) -> Self::Value;
}

//
//  Implementations
//

impl<const N: i32> Exponent for Z<N> {
    type Value = i32;

    fn value(&self) -> Self::Value {
        N
    }
}
