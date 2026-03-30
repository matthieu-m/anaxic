//! An easy to use symbol to represent prefixed symbols without allocations.

use core::fmt;

use crate::api::Symbol;

/// A symbol as a composite of multiple symbols.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CompositeSymbol<F, S> {
    first: F,
    second: S,
}

//
//  Construction/Destruction
//

impl<F, S> CompositeSymbol<F, S> {
    /// Constructs a new instance.
    pub const fn new(first: F, second: S) -> Self {
        Self { first, second }
    }

    /// Returns the parts.
    pub fn into_parts(self) -> (F, S) {
        (self.first, self.second)
    }
}

//
//  Traits
//

impl<F, S> fmt::Display for CompositeSymbol<F, S>
where
    F: fmt::Display,
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}{}", self.first, self.second)
    }
}

impl<F, S> Symbol for CompositeSymbol<F, S>
where
    F: Symbol,
    S: Symbol,
{
}
