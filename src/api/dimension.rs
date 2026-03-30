//! Generic API of dimensions.

use crate::api::{Exponent, Symbol};

/// Dimension.
pub trait Dimension {
    /// The type of the exponent.
    type Exponent: Exponent + 'static;
    /// The type of the symbol.
    type Symbol: Symbol + 'static;

    /// The exponent of the dimension.
    fn exponent(&self) -> Self::Exponent;

    /// Symbol of the dimension.
    fn symbol(&self) -> Self::Symbol;
}
