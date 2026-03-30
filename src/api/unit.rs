//! A unit.
//!
//! The basic brick from which a complete description of a quantity is built.

use crate::api::{Dimension, Exponent, Symbol};

/// A trait to be implemented by a Unit.
///
/// Each Unit must be associated to a specific dimension.
pub trait Unit {
    /// The type of the dimension associated with the unit.
    type Dimension: Dimension<Exponent = Self::Exponent>;
    /// The type of the symbol associated with the unit.
    type Symbol: Symbol + 'static;

    /// The type of the exponent associated with the unit.
    ///
    /// This must match the type of the dimension.
    type Exponent: Exponent + 'static;

    /// Returns a reference to the dimension associated with the unit.
    fn dimension(&self) -> &Self::Dimension;

    /// Returns the symbol of the unit.
    fn symbol(&self) -> Self::Symbol;

    /// Shortcut: returns the exponent of the dimension.
    fn exponent(&self) -> Self::Exponent {
        self.dimension().exponent()
    }
}

/// A trait which expresses how to convert a value expressed in one unit into a value expressed in a different unit.
///
/// This trait should only be implemented for a conversion in the same coordinate system, when the scale is known at
/// compile-time:
///
/// -   It is appropriate to convert from meters to feet, or vice-versa.
/// -   It is NOT appropriate to convert from celsius to fahrenheit, see `TransformPointTo` or `TransformVectorTo`.
/// -   It is NOT appropriate to convert when a run-time context is required, see `ScaleToWith`.
pub trait ScaleTo<Target> {
    /// The type of the scale used in this operation.
    type Scale;

    /// Returns the change of scale necessary to convert from Self to Target.
    fn scale(&self) -> Self::Scale;
}

/// A trait which expresses how to convert a value expressed in one unit into a value expressed in a different unit,
/// in circumstances where this translation requires a runtime context.
///
/// This trait should always be implemented if `ScaleTo` is implemented, for all contexts, you may use
/// the `auto_impl_scale_to_with_context_from_scale` macro to do so.
///
/// For a compile-time version, see `ScaleTo`.
pub trait ScaleToWith<Target, Context> {
    /// The type of the scale used in this operation.
    type Scale;

    /// Returns the change of scale necessary to convert from Self to Target.
    fn scale_with(&self, context: &Context) -> Self::Scale;
}

/// Automatically implements `ScaleToWith` for any `Target` for which `ScaleTo` is implemented.
#[macro_export]
macro_rules! auto_impl_scale_to_with_context_from_scale {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Target, Context> ScaleToWith<Target, Context> for $t
        where
            $t: ScaleTo<Target>,
        {
            type Scale = <$t as ScaleTo<Target>>::Scale;

            fn scale_with(&self, _context: &Context) -> Self::Scale {
                self.scale()
            }
        }
    };
}

/// A trait which expresses how to convert a point expressed in a unit of one coordinate system into a point expressed
/// in a unit of a different coordinate system.
///
/// To transform a vector, see `TransformVectorTo` instead.
///
/// This trait should always be implemented if `ScaleTo` is implemented, you may use the
/// `auto_impl_transform_point_to_from_scale` macro to do so.
///
/// For a run-time version, see `TransformPointToWith`.
pub trait TransformPointTo<Q, Target> {
    /// The resulting type.
    type Output;

    /// Returns the transformed value.
    fn transform_point(&self, value: Q) -> Self::Output;
}

/// A trait which expresses how to convert a point expressed in a unit of one coordinate system into a point expressed
/// in a unit of a different coordinate system.
///
/// To transform a vector, see `TransformVectorToWith` instead.
///
/// This trait should always be implemented if `ScaleToWith` is implemented, you may use the
/// `auto_impl_transform_point_to_with_from_scale` macro to do so.
///
/// This trait should always be implemented if `TransformPointTo` is implemented, for all contexts, you may use the
/// `auto_impl_transform_point_to_with_from_transform` macro to do so.
///
/// For a compile-time version, see `TransformPointTo`.
pub trait TransformPointToWith<Q, Target, Context> {
    /// The resulting type.
    type Output;

    /// Returns the transformed value.
    fn transform_point_with(&self, value: Q, context: &Context) -> Self::Output;
}

/// Automatically implements `TransformPointTo` for any `Target` for which `ScaleTo` is implemented.
#[macro_export]
macro_rules! auto_impl_transform_point_to_from_scale {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Q, Target> TransformPointTo<Q, Target> for $t
        where
            $t: ScaleTo<Target>
            Q: Mul<<$t as ScaleTo<Target>>::Scale>,
        {
            type Output = Q::Output;

            fn transform_point(&self, value: Q) -> Self::Output {
                value * self.scale()
            }
        }
    };
}

/// Automatically implements `TransformPointToWith` for any `Target` for which `ScaleToWith` is implemented.
#[macro_export]
macro_rules! auto_impl_transform_point_to_with_from_scale {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Q, Target, Context> TransformPointToWith<Q, Target, Context> for $t
        where
            $t: ScaleToWith<Target, Context>,
            Q: Mul<<$t as ScaleToWith<Target, Context>>::Scale>,
        {
            type Output = Q::Output;

            fn transform_point_with(&self, value: Q, context: &Context) -> Self::Output {
                value * self.scale_with(context)
            }
        }
    };
}

/// Automatically implements `TransformPointToWith` for any `Target` for which `TransformPointTo` is implemented.
#[macro_export]
macro_rules! auto_impl_transform_point_to_with_from_transform {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Q, Target, Context> TransformPointToWith<Q, Target, Context> for $t
        where
            $t: TransformPointTo<Q, Target>,
        {
            type Output = <$t as TransformPointTo<Q, Target>>::Output;

            fn transform_point_with(&self, value: Q, _context: &Context) -> Self::Output {
                self.transform_point(value)
            }
        }
    };
}

/// A trait which expresses how to convert a vector expressed in a unit of one coordinate system into a vector expressed
/// in a unit of a different coordinate system.
///
/// To transform a point, see `TransformPointTo` instead.
///
/// This trait should always be implemented if `ScaleTo` is implemented, simply returning a 0 offset, you may use
/// the `auto_impl_transform_point_to_from_scale` macro to do so.
///
/// For a run-time version, see `TransformPointToWith`.
pub trait TransformVectorTo<Q, Target> {
    /// The resulting type.
    type Output;

    /// Returns the transformed value.
    fn transform_vector(&self, value: Q) -> Self::Output;
}

/// A trait which expresses how to convert a vector expressed in a unit of one coordinate system into a vector expressed
/// in a unit of a different coordinate system.
///
/// To transform a point, see `TransformPointToWith` instead.
///
/// This trait should always be implemented if `ScaleToWith` is implemented, you may use the
/// `auto_impl_transform_point_to_with_from_scale` macro to do so.
///
/// This trait should always be implemented if `TransformPointTo` is implemented, for all contexts, you may use the
/// `auto_impl_transform_point_to_with_from_transform` macro to do so.
///
/// For a compile-time version, see `TransformPointTo`.
pub trait TransformVectorToWith<Q, Target, Context> {
    /// The resulting type.
    type Output;

    /// Returns the transformed value.
    fn transform_vector_with(&self, value: Q, context: &Context) -> Self::Output;
}

/// Automatically implements `TransformVectorTo` for any `Target` for which `ScaleTo` is implemented.
#[macro_export]
macro_rules! auto_impl_transform_vector_to_from_scale {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Q, Target> TransformVectorTo<Q, Target> for $t
        where
            $t: ScaleTo<Target>
            Q: Mul<<$t as ScaleTo<Target>>::Scale>,
        {
            type Output = Q::Output;

            fn transform_vector(&self, value: Q) -> Self::Output {
                value * self.scale()
            }
        }
    };
}

/// Automatically implements `TransformVectorToWith` for any `Target` for which `ScaleToWith` is implemented.
#[macro_export]
macro_rules! auto_impl_transform_vector_to_with_from_scale {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Q, Target, Context> TransformVectorToWith<Q, Target, Context> for $t
        where
            $t: ScaleToWith<Target, Context>,
            Q: Mul<<$t as ScaleToWith<Target, Context>>::Scale>,
        {
            type Output = Q::Output;

            fn transform_vector_with(&self, value: Q, context: &Context) -> Self::Output {
                value * self.scale_with(context)
            }
        }
    };
}

/// Automatically implements `TransformVectorToWith` for any `Target` for which `TransformPointTo` is implemented.
#[macro_export]
macro_rules! auto_impl_transform_vector_to_with_from_transform {
    //  FIXME: make it work for `$t` with generic components.
    ($t:ty) => {
        impl<Q, Target, Context> TransformVectorToWith<Q, Target, Context> for $t
        where
            $t: TransformPointTo<Q, Target>,
        {
            type Output = <$t as TransformPointTo<Q, Target>>::Output;

            fn transform_vector_with(&self, value: Q, _context: &Context) -> Self::Output {
                self.transform_vector(value)
            }
        }
    };
}
