//! Amount anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::AmountOfSubstanceT},
};

//
//  Mole
//

/// Shortcut to reference the mole anchor, atop which moles are built.
pub type MoleAnchor<const N: i32> = MoleAnchorT<Z<N>>;

/// The underlying mole anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct MoleAnchorT<E>(AmountOfSubstanceT<E>);

impl<const N: i32> MoleAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(AmountOfSubstanceT::new())
    }
}

impl<const N: i32> Unit for MoleAnchorT<Z<N>> {
    type Dimension = AmountOfSubstanceT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "mol"
    }
}

impl_arithmetic_anchor!(MoleAnchorT);
