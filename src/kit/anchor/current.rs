//! Current anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::CurrentT},
};

//
//  Ampere
//

/// Shortcut to reference the ampere anchor, atop which amperes are built.
pub type AmpereAnchor<const N: i32> = AmpereAnchorT<Z<N>>;

/// The underlying ampere anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AmpereAnchorT<E>(CurrentT<E>);

impl<const N: i32> AmpereAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(CurrentT::new())
    }
}

impl<const N: i32> Unit for AmpereAnchorT<Z<N>> {
    type Dimension = CurrentT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "A"
    }
}

impl_arithmetic_anchor!(AmpereAnchorT);
