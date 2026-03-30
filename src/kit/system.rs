//! Helper to implement systems of units.

use core::ops::Mul;

use crate::{
    api::{ScaleTo, ScaleToWith, TransformPointTo, TransformPointToWith, TransformVectorTo, TransformVectorToWith},
    kit::{scaled::Scaled, zero::Zero},
};

/// A system of units.
pub struct System<T> {
    tuple: T,
}

impl<T> System<T> {
    /// Constructs an instance from a tuple.
    pub const fn from_tuple(tuple: T) -> Self {
        Self { tuple }
    }

    /// Returns the inner tuple.
    pub fn into_tuple(self) -> T {
        self.tuple
    }
}

//
//  ScaleTo.
//

impl<T0, U0> ScaleTo<(U0,)> for System<(T0,)>
where
    T0: ScaleTo<U0>,
{
    type Scale = T0::Scale;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale()
    }
}

impl<T0, T1, U0, U1> ScaleTo<(U0, U1)> for System<(T0, T1)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T0::Scale: Mul<T1::Scale>,
{
    type Scale = <T0::Scale as Mul<T1::Scale>>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale()
    }
}

impl<T0, T1, T2, U0, U1, U2> ScaleTo<(U0, U1, U2)> for System<(T0, T1, T2)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T2: ScaleTo<U2>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
{
    type Scale = <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale() * self.tuple.2.scale()
    }
}

impl<T0, T1, T2, T3, U0, U1, U2, U3> ScaleTo<(U0, U1, U2, U3)> for System<(T0, T1, T2, T3)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T2: ScaleTo<U2>,
    T3: ScaleTo<U3>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
{
    type Scale = <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale() * self.tuple.2.scale() * self.tuple.3.scale()
    }
}

impl<T0, T1, T2, T3, T4, U0, U1, U2, U3, U4> ScaleTo<(U0, U1, U2, U3, U4)> for System<(T0, T1, T2, T3, T4)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T2: ScaleTo<U2>,
    T3: ScaleTo<U3>,
    T4: ScaleTo<U4>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
{
    type Scale =
        <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<
            T4::Scale,
        >>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale() * self.tuple.2.scale() * self.tuple.3.scale() * self.tuple.4.scale()
    }
}

impl<T0, T1, T2, T3, T4, T5, U0, U1, U2, U3, U4, U5> ScaleTo<(U0, U1, U2, U3, U4, U5)> for System<(T0, T1, T2, T3, T4, T5)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T2: ScaleTo<U2>,
    T3: ScaleTo<U3>,
    T4: ScaleTo<U4>,
    T5: ScaleTo<U5>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
    <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output: Mul<T5::Scale>,
{
    type Scale = <<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale() * self.tuple.2.scale() * self.tuple.3.scale() * self.tuple.4.scale() * self.tuple.5.scale()
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, U0, U1, U2, U3, U4, U5, U6> ScaleTo<(U0, U1, U2, U3, U4, U5, U6)> for System<(T0, T1, T2, T3, T4, T5, T6)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T2: ScaleTo<U2>,
    T3: ScaleTo<U3>,
    T4: ScaleTo<U4>,
    T5: ScaleTo<U5>,
    T6: ScaleTo<U6>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
    <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output: Mul<T5::Scale>,
    <<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output: Mul<T6::Scale>,
{
    type Scale = <<<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output as Mul<T6::Scale>>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale() * self.tuple.2.scale() * self.tuple.3.scale() * self.tuple.4.scale() * self.tuple.5.scale() * self.tuple.6.scale()
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, U0, U1, U2, U3, U4, U5, U6, U7> ScaleTo<(U0, U1, U2, U3, U4, U5, U6, U7)> for System<(T0, T1, T2, T3, T4, T5, T6, T7)>
where
    T0: ScaleTo<U0>,
    T1: ScaleTo<U1>,
    T2: ScaleTo<U2>,
    T3: ScaleTo<U3>,
    T4: ScaleTo<U4>,
    T5: ScaleTo<U5>,
    T6: ScaleTo<U6>,
    T7: ScaleTo<U7>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
    <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output: Mul<T5::Scale>,
    <<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output: Mul<T6::Scale>,
    <<<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output as Mul<T6::Scale>>::Output: Mul<T7::Scale>,
{
    type Scale = <<<<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output as Mul<T6::Scale>>::Output as Mul<T7::Scale>>::Output;

    fn scale(&self) -> Self::Scale {
        self.tuple.0.scale() * self.tuple.1.scale() * self.tuple.2.scale() * self.tuple.3.scale() * self.tuple.4.scale() * self.tuple.5.scale() * self.tuple.6.scale() * self.tuple.7.scale()
    }
}

//
//  ScaleToWith
//

impl<T0, U0, Context> ScaleToWith<(U0,), Context> for System<(T0,)>
where
    T0: ScaleToWith<(U0,), Context>,
{
    type Scale = T0::Scale;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context)
    }
}

impl<T0, T1, U0, U1, Context> ScaleToWith<(U0, U1), Context> for System<(T0, T1)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T0::Scale: Mul<T1::Scale>,
{
    type Scale = <T0::Scale as Mul<T1::Scale>>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context) * self.tuple.1.scale_with(context)
    }
}

impl<T0, T1, T2, U0, U1, U2, Context> ScaleToWith<(U0, U1, U2), Context> for System<(T0, T1, T2)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T2: ScaleToWith<U2, Context>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
{
    type Scale = <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context) * self.tuple.1.scale_with(context) * self.tuple.2.scale_with(context)
    }
}

impl<T0, T1, T2, T3, U0, U1, U2, U3, Context> ScaleToWith<(U0, U1, U2, U3), Context> for System<(T0, T1, T2, T3)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T2: ScaleToWith<U2, Context>,
    T3: ScaleToWith<U3, Context>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
{
    type Scale = <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context)
            * self.tuple.1.scale_with(context)
            * self.tuple.2.scale_with(context)
            * self.tuple.3.scale_with(context)
    }
}

impl<T0, T1, T2, T3, T4, U0, U1, U2, U3, U4, Context> ScaleToWith<(U0, U1, U2, U3, U4), Context>
    for System<(T0, T1, T2, T3, T4)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T2: ScaleToWith<U2, Context>,
    T3: ScaleToWith<U3, Context>,
    T4: ScaleToWith<U4, Context>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
{
    type Scale =
        <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<
            T4::Scale,
        >>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context)
            * self.tuple.1.scale_with(context)
            * self.tuple.2.scale_with(context)
            * self.tuple.3.scale_with(context)
            * self.tuple.4.scale_with(context)
    }
}

impl<T0, T1, T2, T3, T4, T5, U0, U1, U2, U3, U4, U5, Context> ScaleToWith<(U0, U1, U2, U3, U4, U5), Context> for System<(T0, T1, T2, T3, T4, T5)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T2: ScaleToWith<U2, Context>,
    T3: ScaleToWith<U3, Context>,
    T4: ScaleToWith<U4, Context>,
    T5: ScaleToWith<U5, Context>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
    <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output: Mul<T5::Scale>,
{
    type Scale = <<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context) * self.tuple.1.scale_with(context) * self.tuple.2.scale_with(context) * self.tuple.3.scale_with(context) * self.tuple.4.scale_with(context) * self.tuple.5.scale_with(context)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, U0, U1, U2, U3, U4, U5, U6, Context> ScaleToWith<(U0, U1, U2, U3, U4, U5, U6), Context> for System<(T0, T1, T2, T3, T4, T5, T6)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T2: ScaleToWith<U2, Context>,
    T3: ScaleToWith<U3, Context>,
    T4: ScaleToWith<U4, Context>,
    T5: ScaleToWith<U5, Context>,
    T6: ScaleToWith<U6, Context>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
    <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output: Mul<T5::Scale>,
    <<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output: Mul<T6::Scale>,
{
    type Scale = <<<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output as Mul<T6::Scale>>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context) * self.tuple.1.scale_with(context) * self.tuple.2.scale_with(context) * self.tuple.3.scale_with(context) * self.tuple.4.scale_with(context) * self.tuple.5.scale_with(context) * self.tuple.6.scale_with(context)
    }
}

impl<T0, T1, T2, T3, T4, T5, T6, T7, U0, U1, U2, U3, U4, U5, U6, U7, Context> ScaleToWith<(U0, U1, U2, U3, U4, U5, U6, U7), Context> for System<(T0, T1, T2, T3, T4, T5, T6, T7)>
where
    T0: ScaleToWith<U0, Context>,
    T1: ScaleToWith<U1, Context>,
    T2: ScaleToWith<U2, Context>,
    T3: ScaleToWith<U3, Context>,
    T4: ScaleToWith<U4, Context>,
    T5: ScaleToWith<U5, Context>,
    T6: ScaleToWith<U6, Context>,
    T7: ScaleToWith<U7, Context>,
    T0::Scale: Mul<T1::Scale>,
    <T0::Scale as Mul<T1::Scale>>::Output: Mul<T2::Scale>,
    <<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output: Mul<T3::Scale>,
    <<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output: Mul<T4::Scale>,
    <<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output: Mul<T5::Scale>,
    <<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output: Mul<T6::Scale>,
    <<<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output as Mul<T6::Scale>>::Output: Mul<T7::Scale>,
{
    type Scale = <<<<<<<T0::Scale as Mul<T1::Scale>>::Output as Mul<T2::Scale>>::Output as Mul<T3::Scale>>::Output as Mul<T4::Scale>>::Output as Mul<T5::Scale>>::Output as Mul<T6::Scale>>::Output as Mul<T7::Scale>>::Output;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.tuple.0.scale_with(context) * self.tuple.1.scale_with(context) * self.tuple.2.scale_with(context) * self.tuple.3.scale_with(context) * self.tuple.4.scale_with(context) * self.tuple.5.scale_with(context) * self.tuple.6.scale_with(context) * self.tuple.7.scale_with(context)
    }
}

//
//  Transforms
//

macro_rules! impl_transform {
    ($i:tt; $($pre:ident),*; $($post:ident),*) => {
        impl<Q, TE, TA, UE, UA> TransformPointTo<Q, ($($pre,)* Scaled<UE, UA>, $($post),*)>
            for System<($($pre,)* Scaled<TE, TA>, $($post),*)>
        where
            Scaled<TE, TA>: TransformPointTo<Q, Scaled<UE, UA>>,
        {
            type Output = <Scaled<TE, TA> as TransformPointTo<Q, Scaled<UE, UA>>>::Output;

            fn transform_point(&self, value: Q) -> Self::Output {
                self.tuple.$i.transform_point(value)
            }
        }

        impl<Q, TE, TA, UE, UA, Context> TransformPointToWith<Q, ($($pre,)* Scaled<UE, UA>, $($post),*), Context>
            for System<($($pre,)* Scaled<TE, TA>, $($post),*)>
        where
            Scaled<TE, TA>: TransformPointToWith<Q, Scaled<UE, UA>, Context>,
        {
            type Output = <Scaled<TE, TA> as TransformPointToWith<Q, Scaled<UE, UA>, Context>>::Output;

            fn transform_point_with(&self, value: Q, context: &Context) -> Self::Output {
                self.tuple.$i.transform_point_with(value, context)
            }
        }

        impl<Q, TE, TA, UE, UA> TransformVectorTo<Q, ($($pre,)* Scaled<UE, UA>, $($post),*)>
            for System<($($pre,)* Scaled<TE, TA>, $($post),*)>
        where
            Scaled<TE, TA>: TransformVectorTo<Q, Scaled<UE, UA>>,
        {
            type Output = <Scaled<TE, TA> as TransformVectorTo<Q, Scaled<UE, UA>>>::Output;

            fn transform_vector(&self, value: Q) -> Self::Output {
                self.tuple.$i.transform_vector(value)
            }
        }

        impl<Q, TE, TA, UE, UA, Context> TransformVectorToWith<Q, ($($pre,)* Scaled<UE, UA>, $($post),*), Context>
            for System<($($pre,)* Scaled<TE, TA>, $($post),*)>
        where
            Scaled<TE, TA>: TransformVectorToWith<Q, Scaled<UE, UA>, Context>,
        {
            type Output = <Scaled<TE, TA> as TransformVectorToWith<Q, Scaled<UE, UA>, Context>>::Output;

            fn transform_vector_with(&self, value: Q, context: &Context) -> Self::Output {
                self.tuple.$i.transform_vector_with(value, context)
            }
        }
    };
}

impl_transform!(0;;);

impl_transform!(0;; Zero);
impl_transform!(1; Zero;);

impl_transform!(0;; Zero, Zero);
impl_transform!(1; Zero; Zero);
impl_transform!(2; Zero, Zero;);

impl_transform!(0;; Zero, Zero, Zero);
impl_transform!(1; Zero; Zero, Zero);
impl_transform!(2; Zero, Zero; Zero);
impl_transform!(3; Zero, Zero, Zero;);

impl_transform!(0;; Zero, Zero, Zero, Zero);
impl_transform!(1; Zero; Zero, Zero, Zero);
impl_transform!(2; Zero, Zero; Zero, Zero);
impl_transform!(3; Zero, Zero, Zero; Zero);
impl_transform!(4; Zero, Zero, Zero, Zero;);

impl_transform!(0;; Zero, Zero, Zero, Zero, Zero);
impl_transform!(1; Zero; Zero, Zero, Zero, Zero);
impl_transform!(2; Zero, Zero; Zero, Zero, Zero);
impl_transform!(3; Zero, Zero, Zero; Zero, Zero);
impl_transform!(4; Zero, Zero, Zero, Zero; Zero);
impl_transform!(5; Zero, Zero, Zero, Zero, Zero;);

impl_transform!(0;; Zero, Zero, Zero, Zero, Zero, Zero);
impl_transform!(1; Zero; Zero, Zero, Zero, Zero, Zero);
impl_transform!(2; Zero, Zero; Zero, Zero, Zero, Zero);
impl_transform!(3; Zero, Zero, Zero; Zero, Zero, Zero);
impl_transform!(4; Zero, Zero, Zero, Zero; Zero, Zero);
impl_transform!(5; Zero, Zero, Zero, Zero, Zero; Zero);
impl_transform!(6; Zero, Zero, Zero, Zero, Zero, Zero;);

impl_transform!(0;; Zero, Zero, Zero, Zero, Zero, Zero, Zero);
impl_transform!(1; Zero; Zero, Zero, Zero, Zero, Zero, Zero);
impl_transform!(2; Zero, Zero; Zero, Zero, Zero, Zero, Zero);
impl_transform!(3; Zero, Zero, Zero; Zero, Zero, Zero, Zero);
impl_transform!(4; Zero, Zero, Zero, Zero; Zero, Zero, Zero);
impl_transform!(5; Zero, Zero, Zero, Zero, Zero; Zero, Zero);
impl_transform!(6; Zero, Zero, Zero, Zero, Zero, Zero; Zero);
impl_transform!(7; Zero, Zero, Zero, Zero, Zero, Zero, Zero;);
