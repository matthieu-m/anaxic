//! Angle anchors.

use zahl::Z;

use crate::{
    api::Unit,
    kit::{anchor::Zero, dimension::AngleT},
};

//
//  Degree
//

/// Shortcut to reference the degree anchor, atop which degrees are built.
pub type DegreeAnchor<const N: i32> = DegreeAnchorT<Z<N>>;

/// The underlying radian anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DegreeAnchorT<E>(AngleT<E>);

impl<const N: i32> DegreeAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(AngleT::new())
    }
}

impl<const N: i32> Unit for DegreeAnchorT<Z<N>> {
    type Dimension = AngleT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "°"
    }
}

impl_arithmetic_anchor!(DegreeAnchorT);

//
//  Radian
//

/// Shortcut to reference the radian anchor, atop which radians are built.
pub type RadianAnchor<const N: i32> = RadianAnchorT<Z<N>>;

/// The underlying radian anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RadianAnchorT<E>(AngleT<E>);

impl<const N: i32> RadianAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(AngleT::new())
    }
}

impl<const N: i32> Unit for RadianAnchorT<Z<N>> {
    type Dimension = AngleT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "rad"
    }
}

impl_arithmetic_anchor!(RadianAnchorT);

//
//  Steradian
//

/// Shortcut to reference the steradian anchor, atop which steradians are built.
pub type SteradianAnchor<const N: i32> = SteradianAnchorT<Z<N>>;

/// The underlying steradian anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SteradianAnchorT<E>(AngleT<E>);

impl<const N: i32> SteradianAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(AngleT::new())
    }
}

impl<const N: i32> Unit for SteradianAnchorT<Z<N>> {
    type Dimension = AngleT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "sr"
    }
}

impl_arithmetic_anchor!(SteradianAnchorT);
