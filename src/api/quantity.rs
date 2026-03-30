//! A quantity.

use core::{
    error, fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    str,
};

use crate::api::{
    Scalar, ScaleTo, ScaleToWith, TransformPointTo, TransformPointToWith, TransformVectorTo, TransformVectorToWith,
};

/// Quantity.
///
/// A quantity is, in the end, just a value with an associated unit.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Quantity<V, U> {
    value: V,
    unit: U,
}

//
//  Construction/Destruction.
//

impl<V, U> Quantity<V, U> {
    /// Constructs a new quantity from a value, and a defaulted unit.
    pub fn new(value: V) -> Self
    where
        U: Default,
    {
        Self::from_parts(value, U::default())
    }

    /// Constructs a new quantity from a value and its associated unit.
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

impl<V, U> Quantity<V, U> {
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

impl<V, U> Quantity<Option<V>, U> {
    /// Transposes a `Quantity` of an `Option` into an `Option` of a `Quantity.
    ///
    /// To transpose the unit, see [`transpose_unit`] instead.
    pub fn transpose(self) -> Option<Quantity<V, U>> {
        let value = self.value?;
        let unit = self.unit;

        Some(Quantity { value, unit })
    }
}

impl<V, U> Quantity<V, Option<U>> {
    /// Transposes a `Quantity` of an `Option` into an `Option` of a `Quantity.
    ///
    /// To transpose the value, see [`transpose`] instead.
    pub fn transpose_unit(self) -> Option<Quantity<V, U>> {
        let value = self.value;
        let unit = self.unit?;

        Some(Quantity { value, unit })
    }
}

impl<V, U, E> Quantity<Result<V, E>, U> {
    /// Transposes a `Quantity` of a `Result` into a `Result` of a `Quantity`.
    ///
    /// To transpose the unit, see [`transpose_unit`] instead.
    pub fn transpose(self) -> Result<Quantity<V, U>, E> {
        let value = self.value?;
        let unit = self.unit;

        Ok(Quantity { value, unit })
    }
}

impl<V, U, E> Quantity<V, Result<U, E>> {
    /// Transposes a `Quantity` of a `Result` into a `Result` of a `Quantity`.
    ///
    /// To transpose the value, see [`transpose`] instead.
    pub fn transpose_unit(self) -> Result<Quantity<V, U>, E> {
        let value = self.value;
        let unit = self.unit?;

        Ok(Quantity { value, unit })
    }
}

//
//  Casts to value type.
//

impl<V, U> Quantity<V, U> {
    /// Casts the quantity to a different type of value.
    pub fn cast<OV>(self) -> Quantity<OV, U>
    where
        V: Into<OV>,
    {
        let value = self.value.into();
        let unit = self.unit;

        Quantity { value, unit }
    }

    /// Attempts to cast the quantity to a different type of value.
    pub fn try_cast<OV>(self) -> Result<Quantity<OV, U>, <V as TryInto<OV>>::Error>
    where
        V: TryInto<OV>,
    {
        let value = self.value.try_into()?;
        let unit = self.unit;

        Ok(Quantity { value, unit })
    }
}

//
//  Converts to another unit.
//

impl<V, U> Quantity<V, U> {
    /// Returns a quantity, with the value appropriately scaled for the new unit.
    ///
    /// For scaling with a run-time context, see [`scale_with`] instead.
    ///
    /// For transforming to a different coordinate system, see [`transform`] instead.
    pub fn scale<OU>(self, unit: OU) -> Quantity<V::Output, OU>
    where
        U: ScaleTo<OU>,
        V: Mul<U::Scale>,
    {
        let scale = self.unit.scale();
        let value = self.value * scale;

        Quantity { value, unit }
    }

    /// Returns a quantity, with the value appropriately scaled for the new unit according to the provided context.
    ///
    /// For scaling without a run-time context, see [`scale`] instead.
    ///
    /// For transforming to a different coordinate system, see [`transform_with`] instead.
    pub fn scale_with<OU, C>(self, unit: OU, context: &C) -> Quantity<V::Output, OU>
    where
        U: ScaleToWith<OU, C>,
        V: Mul<U::Scale>,
    {
        let scale = self.unit.scale_with(context);
        let value = self.value * scale;

        Quantity { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system.
    ///
    /// _Note: you should consider using `Point` and `Vector` instead, to avoid applying the wrong transformations._
    ///
    /// For transforming a vector, not a point, see [`transform_vector`] instead.
    ///
    /// For transforming with a run-time context, see [`transform_point_with`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale`] instead.
    pub fn transform_point<OU>(self, unit: OU) -> Quantity<U::Output, OU>
    where
        U: TransformPointTo<V, OU>,
    {
        let value = self.unit.transform_point(self.value);

        Quantity { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system
    /// according to the provided context.
    ///
    /// _Note: you should consider using `Point` and `Vector` instead, to avoid applying the wrong transformations._
    ///
    /// For transforming a vector, not a point, see [`transform_vector_with`] instead.
    ///
    /// For transforming without a run-time context, see [`transform_point`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale_with`] instead.
    pub fn transform_point_with<OU, C>(self, unit: OU, context: &C) -> Quantity<U::Output, OU>
    where
        U: TransformPointToWith<V, OU, C>,
    {
        let value = self.unit.transform_point_with(self.value, context);

        Quantity { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system.
    ///
    /// _Note: you should consider using `Point` and `Vector` instead, to avoid applying the wrong transformations._
    ///
    /// For transforming a point, not a vector, see [`transform_point`] instead.
    ///
    /// For transforming with a run-time context, see [`transform_vector_with`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale`] instead.
    pub fn transform_vector<OU>(self, unit: OU) -> Quantity<U::Output, OU>
    where
        U: TransformVectorTo<V, OU>,
    {
        let value = self.unit.transform_vector(self.value);

        Quantity { value, unit }
    }

    /// Returns a quantity, with the value appropriately transformed for the new unit and its new coordinate system
    /// according to the provided context.
    ///
    /// _Note: you should consider using `Point` and `Vector` instead, to avoid applying the wrong transformations._
    ///
    /// For transforming a point, not a vector, see [`transform_point_with`] instead.
    ///
    /// For transforming without a run-time context, see [`transform_vector`] instead.
    ///
    /// For scaling without switching to a different coordinate system, see [`scale_with`] instead.
    pub fn transform_vector_with<OU, C>(self, unit: OU, context: &C) -> Quantity<U::Output, OU>
    where
        U: TransformVectorToWith<V, OU, C>,
    {
        let value = self.unit.transform_vector_with(self.value, context);

        Quantity { value, unit }
    }
}

//
//  String conversions
//

impl<V, U> fmt::Display for Quantity<V, U>
where
    V: fmt::Display,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl<V, U> str::FromStr for Quantity<V, U>
where
    V: str::FromStr,
    U: str::FromStr,
{
    type Err = ParseQuantityError<V::Err, U::Err>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (value, unit) = s.split_once(' ').ok_or(ParseQuantityError::NoBlank)?;

        let value = value.parse().map_err(ParseQuantityError::Value)?;
        let unit = unit.parse().map_err(ParseQuantityError::Unit)?;

        Ok(Self { value, unit })
    }
}

/// Error which may occur when attempting to parse a quantity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParseQuantityError<V, U> {
    /// The string to parse is ill-formed. The value & unit MUST be separated by a blank space.
    NoBlank,
    /// The value could not be parsed.
    Value(V),
    /// The unit could not be parsed.
    Unit(U),
}

impl<V, U> fmt::Display for ParseQuantityError<V, U>
where
    V: fmt::Display,
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::NoBlank => f.write_str("no blank"),
            Self::Value(error) => write!(f, "failed to parse value with {error}"),
            Self::Unit(error) => write!(f, "failed to parse unit with {error}"),
        }
    }
}

impl<V, U> error::Error for ParseQuantityError<V, U>
where
    V: fmt::Debug + fmt::Display,
    U: fmt::Debug + fmt::Display,
{
}

//
//  Arithmetic: +/-.
//

impl<V, U, OV, OU> AddAssign<Quantity<OV, OU>> for Quantity<V, U>
where
    V: AddAssign<OV>,
    U: AddAssign<OU>,
{
    fn add_assign(&mut self, other: Quantity<OV, OU>) {
        self.value += other.value;
        self.unit += other.unit;
    }
}

impl<V, U, OV, OU> Add<Quantity<OV, OU>> for Quantity<V, U>
where
    V: Add<OV>,
    U: Add<OU>,
{
    type Output = Quantity<V::Output, U::Output>;

    fn add(self, other: Quantity<OV, OU>) -> Self::Output {
        let value = self.value + other.value;
        let unit = self.unit + other.unit;

        Quantity { value, unit }
    }
}

impl<V, U> Neg for Quantity<V, U>
where
    V: Neg,
    U: Neg,
{
    type Output = Quantity<V::Output, U::Output>;

    fn neg(self) -> Self::Output {
        let value = -self.value;
        let unit = -self.unit;

        Quantity { value, unit }
    }
}

impl<V, U, OV, OU> SubAssign<Quantity<OV, OU>> for Quantity<V, U>
where
    V: SubAssign<OV>,
    U: SubAssign<OU>,
{
    fn sub_assign(&mut self, other: Quantity<OV, OU>) {
        self.value -= other.value;
        self.unit -= other.unit;
    }
}

impl<V, U, OV, OU> Sub<Quantity<OV, OU>> for Quantity<V, U>
where
    V: Sub<OV>,
    U: Sub<OU>,
{
    type Output = Quantity<V::Output, U::Output>;

    fn sub(self, other: Quantity<OV, OU>) -> Self::Output {
        let value = self.value - other.value;
        let unit = self.unit - other.unit;

        Quantity { value, unit }
    }
}

//
//  Arithmetic: scaling.
//

impl<V, U, OV> DivAssign<Scalar<OV>> for Quantity<V, U>
where
    V: DivAssign<OV>,
{
    fn div_assign(&mut self, other: Scalar<OV>) {
        self.value /= other.into_value();
    }
}

impl<V, U, OV> Div<Scalar<OV>> for Quantity<V, U>
where
    V: Div<OV>,
{
    type Output = Quantity<V::Output, U>;

    fn div(self, other: Scalar<OV>) -> Self::Output {
        let value = self.value / other.into_value();
        let unit = self.unit;

        Quantity { value, unit }
    }
}

//  FIXME: would need an "Inv" trait to implement scalar / quantity with the inverse of the unit.

impl<V, U, OV> MulAssign<Scalar<OV>> for Quantity<V, U>
where
    V: MulAssign<OV>,
{
    fn mul_assign(&mut self, other: Scalar<OV>) {
        self.value *= other.into_value();
    }
}

impl<V, U, OV> Mul<Scalar<OV>> for Quantity<V, U>
where
    V: Mul<OV>,
{
    type Output = Quantity<<V as Mul<OV>>::Output, U>;

    fn mul(self, other: Scalar<OV>) -> Self::Output {
        let value = self.value * other.into_value();
        let unit = self.unit;

        Quantity { value, unit }
    }
}

impl<V, OV, OU> Mul<Quantity<OV, OU>> for Scalar<V>
where
    V: Mul<OV>,
{
    type Output = Quantity<<V as Mul<OV>>::Output, OU>;

    fn mul(self, other: Quantity<OV, OU>) -> Self::Output {
        let value = self.into_value() * other.value;
        let unit = other.unit;

        Quantity { value, unit }
    }
}

//
//  Arithmetic: combining.
//

impl<V, U, OV, OU> Div<Quantity<OV, OU>> for Quantity<V, U>
where
    V: Div<OV>,
    U: Div<OU>,
{
    type Output = Quantity<V::Output, U::Output>;

    fn div(self, other: Quantity<OV, OU>) -> Self::Output {
        let value = self.value / other.value;
        let unit = self.unit / other.unit;

        Quantity { value, unit }
    }
}

impl<V, U, OV, OU> Mul<Quantity<OV, OU>> for Quantity<V, U>
where
    V: Mul<OV>,
    U: Mul<OU>,
{
    type Output = Quantity<V::Output, U::Output>;

    fn mul(self, other: Quantity<OV, OU>) -> Self::Output {
        let value = self.value * other.value;
        let unit = self.unit * other.unit;

        Quantity { value, unit }
    }
}
