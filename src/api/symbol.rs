//! A symbol for dimensions and units.
//!
//! The easiest type to use for the symbol would be `String`. This is however incompatible with the no-std nature of
//! this crate.

use core::fmt;

/// An abstract symbol.
///
/// The only requirement of a symbol is that it must be displayable.
pub trait Symbol: fmt::Display {}

//
//  Implementation of Symbol for various types.
//

impl Symbol for &str {}

#[cfg(feature = "alloc")]
impl Symbol for alloc::string::String {}
