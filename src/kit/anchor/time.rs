//! Time anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::TimeT},
};

//
//  Second
//

/// Shortcut to reference the second anchor, atop which seconds are built.
pub type SecondAnchor<const N: i32> = SecondAnchorT<Z<N>>;

/// The underlying second anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SecondAnchorT<E>(TimeT<E>);

impl<const N: i32> SecondAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(TimeT::new())
    }
}

impl<const N: i32> Unit for SecondAnchorT<Z<N>> {
    type Dimension = TimeT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "s"
    }
}

impl_arithmetic_anchor!(SecondAnchorT);
