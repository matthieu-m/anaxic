//! length anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::LengthT},
};

//
//  Meter
//

/// Shortcut to reference the meter anchor, atop which meters are built.
pub type MeterAnchor<const N: i32> = MeterAnchorT<Z<N>>;

/// The underlying meter anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MeterAnchorT<E>(LengthT<E>);

impl<const N: i32> MeterAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(LengthT::new())
    }
}

impl<const N: i32> Unit for MeterAnchorT<Z<N>> {
    type Dimension = LengthT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "m"
    }
}

impl_arithmetic_anchor!(MeterAnchorT);
