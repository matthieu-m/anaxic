//! Intensity anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::LuminousIntensityT},
};

//
//  Candela
//

/// Shortcut to reference the candela anchor, atop which candelas are built.
pub type CandelaAnchor<const N: i32> = CandelaAnchorT<Z<N>>;

/// The underlying candela anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CandelaAnchorT<E>(LuminousIntensityT<E>);

impl<const N: i32> CandelaAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(LuminousIntensityT::new())
    }
}

impl<const N: i32> Unit for CandelaAnchorT<Z<N>> {
    type Dimension = LuminousIntensityT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "cd"
    }
}

impl_arithmetic_anchor!(CandelaAnchorT);
