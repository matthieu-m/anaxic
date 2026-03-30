//! A variety of anchor units, atop which derived units can be defined.
//!
//! #   Base units
//!
//! A system of quantities & units such as SI will define _base_ units for each dimension, such as second for time, or
//! kilogram for mass.
//!
//! Unfortunately, the latter doesn't play so _well_ with prefixing scales (kilo, milli). Therefore, this library
//! uses the term _anchor_ unit for the units used as the base of the system.

//  Note: only essential units are defined as anchor right now.

use crate::kit::zero::Zero;

macro_rules! impl_arithmetic_anchor {
    ($name:ident) => {
        impl<E> core::ops::Neg for $name<E> {
            type Output = Self;

            fn neg(self) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::AddAssign for $name<E> {
            fn add_assign(&mut self, _other: Self) {
                //  Empty.
            }
        }

        impl<E> core::ops::Add for $name<E> {
            type Output = Self;

            fn add(self, _other: Self) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::SubAssign for $name<E> {
            fn sub_assign(&mut self, _other: Self) {
                //  Empty.
            }
        }

        impl<E> core::ops::Sub for $name<E> {
            type Output = Self;

            fn sub(self, _other: Self) -> Self::Output {
                self
            }
        }

        impl<E, OE> core::ops::Mul<$name<OE>> for $name<E>
        where
            E: core::ops::Add<OE>,
        {
            type Output = $name<E::Output>;

            fn mul(self, other: $name<OE>) -> Self::Output {
                $name(self.0 * other.0)
            }
        }

        impl<E> core::ops::Mul<Zero> for $name<E> {
            type Output = $name<E>;

            fn mul(self, _other: Zero) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::Mul<$name<E>> for Zero {
            type Output = $name<E>;

            fn mul(self, other: $name<E>) -> Self::Output {
                other
            }
        }

        impl<E, OE> core::ops::Div<$name<OE>> for $name<E>
        where
            E: core::ops::Sub<OE>,
        {
            type Output = $name<E::Output>;

            fn div(self, other: $name<OE>) -> Self::Output {
                $name(self.0 / other.0)
            }
        }

        impl<E> core::ops::Div<Zero> for $name<E> {
            type Output = $name<E>;

            fn div(self, _other: Zero) -> Self::Output {
                self
            }
        }

        impl<E> core::ops::Div<$name<E>> for Zero
        where
            Z<0>: core::ops::Sub<E>,
        {
            type Output = $name<<Z<0> as core::ops::Sub<E>>::Output>;

            fn div(self, other: $name<E>) -> Self::Output {
                type Zero = $name<Z<0>>;

                Zero::new() / other
            }
        }
    };
}

macro_rules! impl_identity_anchor {
    ($t:ident) => {
        impl<const N: i32> $crate::api::ScaleTo<$t<N>> for $t<N> {
            type Scale = $crate::kit::scale::One;

            fn scale(&self) -> Self::Scale {
                $crate::kit::scale::One
            }
        }

        impl<const N: i32, Context> $crate::api::ScaleToWith<$t<N>, Context> for $t<N> {
            type Scale = $crate::kit::scale::One;

            fn scale_with(&self, _context: &Context) -> Self::Scale {
                $crate::kit::scale::One
            }
        }

        impl<Q, const N: i32> $crate::api::TransformPointTo<Q, $t<N>> for $t<N> {
            type Output = Q;

            fn transform_point(&self, value: Q) -> Self::Output {
                value
            }
        }

        impl<Q, const N: i32> $crate::api::TransformVectorTo<Q, $t<N>> for $t<N> {
            type Output = Q;

            fn transform_vector(&self, value: Q) -> Self::Output {
                value
            }
        }

        impl<Q, const N: i32, Context> $crate::api::TransformPointToWith<Q, $t<N>, Context> for $t<N> {
            type Output = Q;

            fn transform_point_with(&self, value: Q, _context: &Context) -> Self::Output {
                value
            }
        }

        impl<Q, const N: i32, Context> $crate::api::TransformVectorToWith<Q, $t<N>, Context> for $t<N> {
            type Output = Q;

            fn transform_vector_with(&self, value: Q, _context: &Context) -> Self::Output {
                value
            }
        }
    };
}

mod amount;
mod angle;
mod current;
mod intensity;
mod length;
mod mass;
mod temperature;
mod time;

pub use amount::{MoleAnchor, MoleAnchorT};
pub use angle::{DegreeAnchor, DegreeAnchorT, RadianAnchor, RadianAnchorT, SteradianAnchor, SteradianAnchorT};
pub use current::{AmpereAnchor, AmpereAnchorT};
pub use intensity::{CandelaAnchor, CandelaAnchorT};
pub use length::{MeterAnchor, MeterAnchorT};
pub use mass::{GramAnchor, GramAnchorT};
pub use temperature::{
    CelsiusAnchor, CelsiusAnchorT, FahrenheitAnchor, FahrenheitAnchorT, KelvinAnchor, KelvinAnchorT,
};
pub use time::{SecondAnchor, SecondAnchorT};
