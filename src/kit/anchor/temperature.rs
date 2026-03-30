//! Temperature anchors.

use core::{
    num::NonZeroI64,
    ops::{Add, Mul},
};

use zahl::Z;

use crate::{
    api::{TransformPointTo, TransformPointToWith, TransformVectorTo, TransformVectorToWith, Unit},
    kit::{
        anchor::Zero,
        dimension::TemperatureT,
        scale::{Integral, IntegralRatio},
    },
};

//
//  Celsius
//

/// Shortcut to reference the celsius anchor, atop which celsius degrees are built.
pub type CelsiusAnchor<const N: i32> = CelsiusAnchorT<Z<N>>;

/// The underlying celsius anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CelsiusAnchorT<E>(TemperatureT<E>);

impl<const N: i32> CelsiusAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(TemperatureT::new())
    }
}

impl<const N: i32> Unit for CelsiusAnchorT<Z<N>> {
    type Dimension = TemperatureT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "°C"
    }
}

impl_arithmetic_anchor!(CelsiusAnchorT);
impl_identity_anchor!(CelsiusAnchor);

//
//  Fahrenheit
//

/// Shortcut to reference the fahrenheit anchor, atop which fahrenheit degrees are built.
pub type FahrenheitAnchor<const N: i32> = FahrenheitAnchorT<Z<N>>;

/// The underlying fahrenheit anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FahrenheitAnchorT<E>(TemperatureT<E>);

impl<const N: i32> FahrenheitAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(TemperatureT::new())
    }
}

impl<const N: i32> Unit for FahrenheitAnchorT<Z<N>> {
    type Dimension = TemperatureT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "°F"
    }
}

impl_arithmetic_anchor!(FahrenheitAnchorT);
impl_identity_anchor!(FahrenheitAnchor);

//
//  Kelvin
//

/// Shortcut to reference the kelvin anchor, atop which kelvin degrees are built.
pub type KelvinAnchor<const N: i32> = KelvinAnchorT<Z<N>>;

/// The underlying kelvin anchor.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct KelvinAnchorT<E>(TemperatureT<E>);

impl<const N: i32> KelvinAnchorT<Z<N>> {
    /// Constructs a new instance.
    pub const fn new() -> Self {
        Self(TemperatureT::new())
    }
}

impl<const N: i32> Unit for KelvinAnchorT<Z<N>> {
    type Dimension = TemperatureT<Z<N>>;
    type Exponent = Z<N>;
    type Symbol = &'static str;

    fn dimension(&self) -> &Self::Dimension {
        &self.0
    }

    fn symbol(&self) -> Self::Symbol {
        "K"
    }
}

impl_arithmetic_anchor!(KelvinAnchorT);
impl_identity_anchor!(KelvinAnchor);

//
//  Conversions
//

//  FIXME:
//  -   Should direct Fahrenheit <-> Kelvin be implemented?
//  -   Should conversions also be implemented for a power of -1?

const FIVE: NonZeroI64 = NonZeroI64::new(5).unwrap();
const NINE: NonZeroI64 = NonZeroI64::new(9).unwrap();
const THIRTY_TWO: NonZeroI64 = NonZeroI64::new(32).unwrap();

const CELSIUS_TO_FAHRENHEIT_RATIO: IntegralRatio = IntegralRatio::new(NINE, FIVE);
const CELSIUS_TO_FAHRENHEIT_OFFSET: Integral = Integral::new(THIRTY_TWO);

const CELSIUS_TO_KELVIN_OFFSET: IntegralRatio =
    IntegralRatio::new(NonZeroI64::new(27_315).unwrap(), NonZeroI64::new(100).unwrap());

//  Celsius to Fahrenheit

impl<Q> TransformPointTo<Q, FahrenheitAnchor<1>> for CelsiusAnchor<1>
where
    Q: Mul<IntegralRatio>,
    Q::Output: Add<Integral>,
{
    type Output = <Q::Output as Add<Integral>>::Output;

    fn transform_point(&self, value: Q) -> Self::Output {
        value * CELSIUS_TO_FAHRENHEIT_RATIO + CELSIUS_TO_FAHRENHEIT_OFFSET
    }
}

impl<Q> TransformVectorTo<Q, FahrenheitAnchor<1>> for CelsiusAnchor<1>
where
    Q: Mul<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_vector(&self, value: Q) -> Self::Output {
        value * CELSIUS_TO_FAHRENHEIT_RATIO
    }
}

impl<Q, Context> TransformPointToWith<Q, FahrenheitAnchor<1>, Context> for CelsiusAnchor<1>
where
    Q: Mul<IntegralRatio>,
    Q::Output: Add<Integral>,
{
    type Output = <Q::Output as Add<Integral>>::Output;

    fn transform_point_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformPointTo<Q, FahrenheitAnchor<1>>>::transform_point(self, value)
    }
}

impl<Q, Context> TransformVectorToWith<Q, FahrenheitAnchor<1>, Context> for CelsiusAnchor<1>
where
    Q: Mul<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_vector_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformVectorTo<Q, FahrenheitAnchor<1>>>::transform_vector(self, value)
    }
}

//  Fahrenheit to Celsius

impl<Q> TransformPointTo<Q, CelsiusAnchor<1>> for FahrenheitAnchor<1>
where
    Q: Add<Integral>,
    Q::Output: Mul<IntegralRatio>,
{
    type Output = <Q::Output as Mul<IntegralRatio>>::Output;

    fn transform_point(&self, value: Q) -> Self::Output {
        (value + -CELSIUS_TO_FAHRENHEIT_OFFSET) * CELSIUS_TO_FAHRENHEIT_RATIO.invert()
    }
}

impl<Q> TransformVectorTo<Q, CelsiusAnchor<1>> for FahrenheitAnchor<1>
where
    Q: Mul<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_vector(&self, value: Q) -> Self::Output {
        value * CELSIUS_TO_FAHRENHEIT_RATIO.invert()
    }
}

impl<Q, Context> TransformPointToWith<Q, CelsiusAnchor<1>, Context> for FahrenheitAnchor<1>
where
    Q: Add<Integral>,
    Q::Output: Mul<IntegralRatio>,
{
    type Output = <Q::Output as Mul<IntegralRatio>>::Output;

    fn transform_point_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformPointTo<Q, CelsiusAnchor<1>>>::transform_point(self, value)
    }
}

impl<Q, Context> TransformVectorToWith<Q, CelsiusAnchor<1>, Context> for FahrenheitAnchor<1>
where
    Q: Mul<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_vector_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformVectorTo<Q, CelsiusAnchor<1>>>::transform_vector(self, value)
    }
}

//  Celsius to Kelvin

impl<Q> TransformPointTo<Q, KelvinAnchor<1>> for CelsiusAnchor<1>
where
    Q: Add<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_point(&self, value: Q) -> Self::Output {
        value + CELSIUS_TO_KELVIN_OFFSET
    }
}

impl<Q> TransformVectorTo<Q, KelvinAnchor<1>> for CelsiusAnchor<1> {
    type Output = Q;

    fn transform_vector(&self, value: Q) -> Self::Output {
        value
    }
}

impl<Q, Context> TransformPointToWith<Q, KelvinAnchor<1>, Context> for CelsiusAnchor<1>
where
    Q: Add<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_point_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformPointTo<Q, KelvinAnchor<1>>>::transform_point(self, value)
    }
}

impl<Q, Context> TransformVectorToWith<Q, KelvinAnchor<1>, Context> for CelsiusAnchor<1> {
    type Output = Q;

    fn transform_vector_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformVectorTo<Q, KelvinAnchor<1>>>::transform_vector(self, value)
    }
}

//  Kelvin to Celsius

impl<Q> TransformPointTo<Q, CelsiusAnchor<1>> for KelvinAnchor<1>
where
    Q: Add<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_point(&self, value: Q) -> Self::Output {
        value + -CELSIUS_TO_KELVIN_OFFSET
    }
}

impl<Q> TransformVectorTo<Q, CelsiusAnchor<1>> for KelvinAnchor<1> {
    type Output = Q;

    fn transform_vector(&self, value: Q) -> Self::Output {
        value
    }
}

impl<Q, Context> TransformPointToWith<Q, CelsiusAnchor<1>, Context> for KelvinAnchor<1>
where
    Q: Add<IntegralRatio>,
{
    type Output = Q::Output;

    fn transform_point_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformPointTo<Q, CelsiusAnchor<1>>>::transform_point(self, value)
    }
}

impl<Q, Context> TransformVectorToWith<Q, CelsiusAnchor<1>, Context> for KelvinAnchor<1> {
    type Output = Q;

    fn transform_vector_with(&self, value: Q, _context: &Context) -> Self::Output {
        <Self as TransformVectorTo<Q, CelsiusAnchor<1>>>::transform_vector(self, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CELSIUS: CelsiusAnchor<1> = CelsiusAnchor::new();
    const FAHRENHEIT: FahrenheitAnchor<1> = FahrenheitAnchor::new();
    const KELVIN: KelvinAnchor<1> = KelvinAnchor::new();

    #[test]
    fn celsius_fahrenheit() {
        #[track_caller]
        fn assert_c_f_point(celsius: f64, fahrenheit: f64) {
            assert_eq!(fahrenheit, point(CELSIUS).to_fahrenheit(celsius));
            assert_eq!(celsius, point(FAHRENHEIT).to_celsius(fahrenheit));
        }

        #[track_caller]
        fn assert_c_f_vector(celsius: f64, fahrenheit: f64) {
            assert_eq!(fahrenheit, vector(CELSIUS).to_fahrenheit(celsius));
            assert_eq!(celsius, vector(FAHRENHEIT).to_celsius(fahrenheit));
        }

        assert_c_f_point(0., 32.);
        assert_c_f_point(100., 212.);

        assert_c_f_vector(0., 0.);
        assert_c_f_vector(1., 1.8);
        assert_c_f_vector(100., 180.);
    }

    #[test]
    fn celsius_to_kelvin() {
        #[track_caller]
        fn assert_c_k_point(celsius: f64, kelvin: f64) {
            assert_eq!(kelvin, point(CELSIUS).to_kelvin(celsius));
            assert_eq!(celsius, point(KELVIN).to_celsius(kelvin));
        }

        #[track_caller]
        fn assert_c_k_vector(celsius: f64, kelvin: f64) {
            assert_eq!(kelvin, vector(CELSIUS).to_kelvin(celsius));
            assert_eq!(celsius, vector(KELVIN).to_celsius(kelvin));
        }

        assert_c_k_point(0., 273.15);
        assert_c_k_point(-273.15, 0.);

        assert_c_k_vector(0., 0.);
        assert_c_k_vector(1., 1.);
        assert_c_k_vector(100., 100.);
    }

    fn point<T>(anchor: T) -> PointTransformer<T> {
        PointTransformer::new(anchor)
    }

    fn vector<T>(anchor: T) -> VectorTransformer<T> {
        VectorTransformer::new(anchor)
    }

    struct PointTransformer<T> {
        anchor: T,
    }

    impl<T> PointTransformer<T> {
        fn new(anchor: T) -> Self {
            Self { anchor }
        }

        fn to_celsius(&self, value: f64) -> f64
        where
            T: TransformPointTo<f64, CelsiusAnchor<1>, Output = f64>,
        {
            self.anchor.transform_point(value)
        }

        fn to_fahrenheit(&self, value: f64) -> f64
        where
            T: TransformPointTo<f64, FahrenheitAnchor<1>, Output = f64>,
        {
            self.anchor.transform_point(value)
        }

        fn to_kelvin(&self, value: f64) -> f64
        where
            T: TransformPointTo<f64, KelvinAnchor<1>, Output = f64>,
        {
            self.anchor.transform_point(value)
        }
    }

    struct VectorTransformer<T> {
        anchor: T,
    }

    impl<T> VectorTransformer<T> {
        fn new(anchor: T) -> Self {
            Self { anchor }
        }

        fn to_celsius(&self, value: f64) -> f64
        where
            T: TransformVectorTo<f64, CelsiusAnchor<1>, Output = f64>,
        {
            self.anchor.transform_vector(value)
        }

        fn to_fahrenheit(&self, value: f64) -> f64
        where
            T: TransformVectorTo<f64, FahrenheitAnchor<1>, Output = f64>,
        {
            self.anchor.transform_vector(value)
        }

        fn to_kelvin(&self, value: f64) -> f64
        where
            T: TransformVectorTo<f64, KelvinAnchor<1>, Output = f64>,
        {
            self.anchor.transform_vector(value)
        }
    }
} // mod tests
