//! Mass anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::MassT},
};

//
//  Gram
//

/// Shortcut to reference the gram anchor, atop which grams are built.
pub type GramAnchor<const N: i32> = GramAnchorT<Z<N>>;

/// The underlying gram anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GramAnchorT<E>(MassT<E>);

impl<const N: i32> GramAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(MassT::new())
    }
}

impl<const N: i32> Unit for GramAnchorT<Z<N>> {
    type Dimension = MassT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "g"
    }
}

impl_arithmetic_anchor!(GramAnchorT);
