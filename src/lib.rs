//! A library of quantities and units.

#![cfg_attr(not(test), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod api;
pub mod kit;
pub mod system;
