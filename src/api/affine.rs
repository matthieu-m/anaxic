//! Affine spaces for quantities.
//!
//! A number of quantities are often best represented as affine value: points & vectors (displacements). For
//! example, temperature may be expressed:
//!
//! -   In Kelvin.
//! -   In Celsius, which is the same scale as Kelvin, by displaced by 273.15°.
//! -   In Fahrenheit, which is both a different scale & origin.
//!
//! The current temperature in °F is converted to °C by applying the formula: °C = (°F - 32) * 5/9,
//! however a jump in temperature of 10°F is converted to °C by applying the formula: °C = °F * 5/9.
//!
//! And that is why the difference between points & vectors matter.

use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use crate::api::{
    Scalar, ScaleTo, ScaleToWith, TransformPointTo, TransformPointToWith, TransformVectorTo, TransformVectorToWith,
};

/// A point in a specific unit, of a specific coordinate system.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Point<V, U> {
    value: V,
    unit: U,
}

/// A vector in a specific unit.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Vector<V, U> {
    value: V,
    unit: U,
}

//
//  Construction/Destruction.
//

impl<V, U> Point<V, U> {
    /// Constructs a new point from its value, and a defaulted unit.
    pub fn new(value: V) -> Self
    where
        U: Default,
    {
        Self::from_parts(value, U::default())
    }

    /// Constructs a new point from its value and unit.
    pub const fn from_parts(value: V, unit: U) -> Self {
        Self { value, unit }
    }

    /// Returns the parts.
    pub fn into_parts(self) -> (V, U) {
        let Self { value, unit } = self;

        (value, unit)
    }
}

impl<V, U> Vector<V, U> {
    /// Constructs a new vector from its value, and a defaulted unit.
    pub fn new(value: V) -> Self
    where
        U: Default,
    {
        Self::from_parts(value, U::default())
    }

    /// Constructs a new vector from its value and unit.
    pub const fn from_parts(value: V, unit: U) -> Self {
        Self { value, unit }
    }

    /// Returns the parts.
    pub fn into_parts(self) -> (V, U) {
        let Self { value, unit } = self;

        (value, unit)
    }
}

//
//  Getters.
//

impl<V, U> Point<V, U> {
    /// Returns a reference to the value.
    pub const fn as_value(&self) -> &V {
        &self.value
    }

    /// Returns a mutable reference to the value.
    pub const fn as_value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    /// Returns the value.
    pub fn into_value(self) -> V {
        self.value
    }

    /// Returns a reference to the unit.
    pub const fn as_unit(&self) -> &U {
        &self.unit
    }

    /// Returns a mutable reference to the unit.
    pub const fn as_unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    /// Returns the unit.
    pub fn into_unit(self) -> U {
        self.unit
    }
}

impl<V, U> Vector<V, U> {
    /// Returns a reference to the value.
    pub const fn as_value(&self) -> &V {
        &self.value
    }

    /// Returns a mutable reference to the value.
    pub const fn as_value_mut(&mut self) -> &mut V {
        &mut self.value
    }

    /// Returns the value.
    pub fn into_value(self) -> V {
        self.value
    }

    /// Returns a reference to the unit.
    pub const fn as_unit(&self) -> &U {
        &self.unit
    }

    /// Returns a mutable reference to the unit.
    pub const fn as_unit_mut(&mut self) -> &mut U {
        &mut self.unit
    }

    /// Returns the unit.
    pub fn into_unit(self) -> U {
        self.unit
    }
}

//
//  Tranpositions: Option/Result.
//

impl<V, U> Point<Option<V>, U> {
    /// Transposes a `Point` of an `Option` into an `Option` of a `Point.
    ///
    /// To transpose the unit, see [`transpose_unit`] instead.
    pub fn transpose(self) -> Option<Point<V, U>> {
        let value = self.value?;
        let unit = self.unit;

        Some(Point { value, unit })
    }
}

impl<V, U> Vector<Option<V>, U> {
    /// Transposes a `Vector` of an `Option` into an `Option` of a `Vector.
    ///
    /// To transpose the unit, see [`transpose_unit`] instead.
    pub fn transpose(self) -> Option<Vector<V, U>> {
        let value = self.value?;
        let unit = self.unit;

        Some(Vector { value, unit })
    }
}

impl<V, U> Point<V, Option<U>> {
    /// Transposes a `Point` of an `Option` into an `Option` of a `Point.
    ///
    /// To transpose the value, see [`transpose`] instead.
    pub fn transpose_unit(self) -> Option<Point<V, U>> {
        let value = self.value;
        let unit = self.unit?;

        Some(Point { value, unit })
    }
}

impl<V, U> Vector<V, Option<U>> {
    /// Transposes a `Vector` of an `Option` into an `Option` of a `Vector.
    ///
    /// To transpose the value, see [`transpose`] instead.
    pub fn transpose_unit(self) -> Option<Vector<V, U>> {
        let value = self.value;
        let unit = self.unit?;

        Some(Vector { value, unit })
    }
}

impl<V, U, E> Point<Result<V, E>, U> {
    /// Transposes a `Point` of a `Result` into a `Result` of a `Point`.
    ///
    /// To transpose the unit, see [`transpose_unit`] instead.
    pub fn transpose(self) -> Result<Point<V, U>, E> {
        let value = self.value?;
        let unit = self.unit;

        Ok(Point { value, unit })
    }
}

impl<V, U, E> Vector<Result<V, E>, U> {
    /// Transposes a `Vector` of a `Result` into a `Result` of a `Vector`.
    ///
    /// To transpose the unit, see [`transpose_unit`] instead.
    pub fn transpose(self) -> Result<Vector<V, U>, E> {
        let value = self.value?;
        let unit = self.unit;

        Ok(Vector { value, unit })
    }
}

impl<V, U, E> Point<V, Result<U, E>> {
    /// Transposes a `Point` of a `Result` into a `Result` of a `Point`.
    ///
    /// To transpose the value, see [`transpose`] instead.
    pub fn transpose_unit(self) -> Result<Point<V, U>, E> {
        let value = self.value;
        let unit = self.unit?;

        Ok(Point { value, unit })
    }
}

impl<V, U, E> Vector<V, Result<U, E>> {
    /// Transposes a `Vector` of a `Result` into a `Result` of a `Vector`.
    ///
    /// To transpose the value, see [`transpose`] instead.
    pub fn transpose_unit(self) -> Result<Vector<V, U>, E> {
        let value = self.value;
        let unit = self.unit?;

        Ok(Vector { value, unit })
    }
}

//
//  Casts to value type.
//

impl<V, U> Point<V, U> {
    /// Casts the quantity to a different type of value.
    pub fn cast<OV>(self) -> Point<OV, U>
    where
        V: Into<OV>,
    {
        let value = self.value.into();
        let unit = self.unit;

        Point { value, unit }
    }

    /// Attempts to cast the quantity to a different type of value.
    pub fn try_cast<OV>(self) -> Result<Point<OV, U>, <V as TryInto<OV>>::Error>
    where
        V: TryInto<OV>,
    {
        let value = self.value.try_into()?;
        let unit = self.unit;

        Ok(Point { value, unit })
    }
}

impl<V, U> Vector<V, U> {
    /// Casts the quantity to a different type of value.
    pub fn cast<OV>(self) -> Vector<OV, U>
    where
        V: Into<OV>,
    {
        let value = self.value.into();
        let unit = self.unit;

        Vector { value, unit }
    }

    /// Attempts to cast the quantity to a different type of value.
    pub fn try_cast<OV>(self) -> Result<Vector<OV, U>, <V as TryInto<OV>>::Error>
    where
        V: TryInto<OV>,
    {
        let value = self.value.try_into()?;
        let unit = self.unit;

        Ok(Vector { value, unit })
    }
}

//
//  Converts to another unit.
//

impl<V, U> Point<V, U> {
    /// Returns a quantity, with the value appropriately scaled for the new unit.
    ///
    /// For scaling with a run-time context, see [`scale_with`] instead.
    ///
    /// For transforming to a different coordinate system, see [`transform`] instead.
    pub fn scale<OU>(self, unit: OU) -> Point<V::Output, OU>
    where
        U: ScaleTo<OU>,
        V: Mul<U::Scale>,
    {
        let scale = self.unit.scale();
        let value = self.value * scale;

        Point { value, unit }
    }

    /// Returns a quantity, with the value appropriately scaled for the new unit according to the provided context.
    ///
    /// For scaling without a run-time context, see [`scale`] instead.
    ///
    /// For transforming to a different coordinate system, see [`transform_with`] instead.
    pub fn scale_with<OU, C>(self, unit: OU, context: &C) -> Point<V::Output, OU>
    where
        U: ScaleToWith<OU, C>,
        V: Mul<U::Scale>,
    {
        let scale = self.unit.scale_with(context);
        let value = self.value * scale;

        Point { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system.
    ///
    /// For transforming with a run-time context, see [`transform_with`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale`] instead.
    pub fn transform<OU>(self, unit: OU) -> Point<U::Output, OU>
    where
        U: TransformPointTo<V, OU>,
    {
        let value = self.unit.transform_point(self.value);

        Point { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system
    /// according to the provided context.
    ///
    /// For transforming without a run-time context, see [`transform`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale_with`] instead.
    pub fn transform_with<OU, C>(self, unit: OU, context: &C) -> Point<U::Output, OU>
    where
        U: TransformPointToWith<V, OU, C>,
    {
        let value = self.unit.transform_point_with(self.value, context);

        Point { value, unit }
    }
}

impl<V, U> Vector<V, U> {
    /// Returns a quantity, with the value appropriately scaled for the new unit.
    ///
    /// For scaling with a run-time context, see [`scale_with`] instead.
    ///
    /// For transforming to a different coordinate system, see [`transform`] instead.
    pub fn scale<OU>(self, unit: OU) -> Vector<V::Output, OU>
    where
        U: ScaleTo<OU>,
        V: Mul<U::Scale>,
    {
        let scale = self.unit.scale();
        let value = self.value * scale;

        Vector { value, unit }
    }

    /// Returns a quantity, with the value appropriately scaled for the new unit according to the provided context.
    ///
    /// For scaling without a run-time context, see [`scale`] instead.
    ///
    /// For transforming to a different coordinate system, see [`transform_with`] instead.
    pub fn scale_with<OU, C>(self, unit: OU, context: &C) -> Vector<V::Output, OU>
    where
        U: ScaleToWith<OU, C>,
        V: Mul<U::Scale>,
    {
        let scale = self.unit.scale_with(context);
        let value = self.value * scale;

        Vector { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system.
    ///
    /// For transforming with a run-time context, see [`transform_with`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale`] instead.
    pub fn transform<OU>(self, unit: OU) -> Vector<U::Output, OU>
    where
        U: TransformVectorTo<V, OU>,
    {
        let value = self.unit.transform_vector(self.value);

        Vector { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system
    /// according to the provided context.
    ///
    /// For transforming without a run-time context, see [`transform`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale_with`] instead.
    pub fn transform_with<OU, C>(self, unit: OU, context: &C) -> Vector<U::Output, OU>
    where
        U: TransformVectorToWith<V, OU, C>,
    {
        let value = self.unit.transform_vector_with(self.value, context);

        Vector { value, unit }
    }
}

//
//  Point Arithmetic: +/-.
//

impl<V, U, OV, OU> AddAssign<Vector<OV, OU>> for Point<V, U>
where
    V: AddAssign<OV>,
    U: AddAssign<OU>,
{
    fn add_assign(&mut self, other: Vector<OV, OU>) {
        self.value += other.value;
        self.unit += other.unit;
    }
}

impl<V, U, OV, OU> Add<Vector<OV, OU>> for Point<V, U>
where
    V: Add<OV>,
    U: Add<OU>,
{
    type Output = Point<V::Output, U::Output>;

    fn add(self, other: Vector<OV, OU>) -> Self::Output {
        let value = self.value + other.value;
        let unit = self.unit + other.unit;

        Point { value, unit }
    }
}

impl<V, U, OV, OU> SubAssign<Vector<OV, OU>> for Point<V, U>
where
    V: SubAssign<OV>,
    U: SubAssign<OU>,
{
    fn sub_assign(&mut self, other: Vector<OV, OU>) {
        self.value -= other.value;
        self.unit -= other.unit;
    }
}

impl<V, U, OV, OU> Sub<Point<OV, OU>> for Point<V, U>
where
    V: Sub<OV>,
    U: Sub<OU>,
{
    type Output = Vector<V::Output, U::Output>;

    fn sub(self, other: Point<OV, OU>) -> Self::Output {
        let value = self.value - other.value;
        let unit = self.unit - other.unit;

        Vector { value, unit }
    }
}

//
//  Vector Arithmetic: +/-.
//

impl<V, U, OV, OU> AddAssign<Vector<OV, OU>> for Vector<V, U>
where
    V: AddAssign<OV>,
    U: AddAssign<OU>,
{
    fn add_assign(&mut self, other: Vector<OV, OU>) {
        self.value += other.value;
        self.unit += other.unit;
    }
}

impl<V, U, OV, OU> Add<Vector<OV, OU>> for Vector<V, U>
where
    V: Add<OV>,
    U: Add<OU>,
{
    type Output = Vector<V::Output, U::Output>;

    fn add(self, other: Vector<OV, OU>) -> Self::Output {
        let value = self.value + other.value;
        let unit = self.unit + other.unit;

        Vector { value, unit }
    }
}

impl<V, U> Neg for Vector<V, U>
where
    V: Neg,
{
    type Output = Vector<V::Output, U>;

    fn neg(self) -> Self::Output {
        let value = -self.value;

        Vector { value, unit: self.unit }
    }
}

impl<V, U, OV, OU> SubAssign<Vector<OV, OU>> for Vector<V, U>
where
    V: SubAssign<OV>,
    U: SubAssign<OU>,
{
    fn sub_assign(&mut self, other: Vector<OV, OU>) {
        self.value -= other.value;
        self.unit -= other.unit;
    }
}

impl<V, U, OV, OU> Sub<Vector<OV, OU>> for Vector<V, U>
where
    V: Sub<OV>,
    U: Sub<OU>,
{
    type Output = Vector<V::Output, U::Output>;

    fn sub(self, other: Vector<OV, OU>) -> Self::Output {
        let value = self.value - other.value;
        let unit = self.unit - other.unit;

        Vector { value, unit }
    }
}

//
//  Vector Arithmetic: scaling.
//

impl<V, U, OV> DivAssign<Scalar<OV>> for Vector<V, U>
where
    V: DivAssign<OV>,
{
    fn div_assign(&mut self, other: Scalar<OV>) {
        self.value /= other.into_value();
    }
}

impl<V, U, OV> Div<Scalar<OV>> for Vector<V, U>
where
    V: Div<OV>,
{
    type Output = Vector<V::Output, U>;

    fn div(self, other: Scalar<OV>) -> Self::Output {
        let value = self.value / other.into_value();
        let unit = self.unit;

        Vector { value, unit }
    }
}

//  FIXME: would need an "Inv" trait to implement scalar / vector with the inverse of the unit.

impl<V, U, OV> MulAssign<Scalar<OV>> for Vector<V, U>
where
    V: MulAssign<OV>,
{
    fn mul_assign(&mut self, other: Scalar<OV>) {
        self.value *= other.into_value();
    }
}

impl<V, U, OV> Mul<Scalar<OV>> for Vector<V, U>
where
    V: Mul<OV>,
{
    type Output = Vector<<V as Mul<OV>>::Output, U>;

    fn mul(self, other: Scalar<OV>) -> Self::Output {
        let value = self.value * other.into_value();
        let unit = self.unit;

        Vector { value, unit }
    }
}

impl<V, OV, OU> Mul<Vector<OV, OU>> for Scalar<V>
where
    V: Mul<OV>,
{
    type Output = Vector<<V as Mul<OV>>::Output, OU>;

    fn mul(self, other: Vector<OV, OU>) -> Self::Output {
        let value = self.into_value() * other.value;
        let unit = other.unit;

        Vector { value, unit }
    }
}

//
//  Vector Arithmetic: *//.
//

impl<V, U, OV, OU> DivAssign<Vector<OV, OU>> for Vector<V, U>
where
    V: DivAssign<OV>,
    U: DivAssign<OU>,
{
    fn div_assign(&mut self, other: Vector<OV, OU>) {
        self.value /= other.value;
        self.unit /= other.unit;
    }
}

impl<V, U, OV, OU> Div<Vector<OV, OU>> for Vector<V, U>
where
    V: Div<OV>,
    U: Div<OU>,
{
    type Output = Vector<V::Output, U::Output>;

    fn div(self, other: Vector<OV, OU>) -> Self::Output {
        let value = self.value / other.value;
        let unit = self.unit / other.unit;

        Vector { value, unit }
    }
}

impl<V, U, OV, OU> MulAssign<Vector<OV, OU>> for Vector<V, U>
where
    V: MulAssign<OV>,
    U: MulAssign<OU>,
{
    fn mul_assign(&mut self, other: Vector<OV, OU>) {
        self.value *= other.value;
        self.unit *= other.unit;
    }
}

impl<V, U, OV, OU> Mul<Vector<OV, OU>> for Vector<V, U>
where
    V: Mul<OV>,
    U: Mul<OU>,
{
    type Output = Vector<V::Output, U::Output>;

    fn mul(self, other: Vector<OV, OU>) -> Self::Output {
        let value = self.value * other.value;
        let unit = self.unit * other.unit;

        Vector { value, unit }
    }
}
