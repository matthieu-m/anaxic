//! Dimensions.

use core::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{
    api::{Exponent, Symbol},
    kit::{
        dimension::{
            AmountOfSubstanceDimension, AngleDimension, CurrentDimension, LengthDimension, LuminousIntensityDimension,
            MassDimension, TemperatureDimension, TimeDimension,
        },
        format::format_dimensions,
    },
};

/// Dimensions of the SI system.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SiDimensions<A, L, M, T, I, O, J, N> {
    /// Angle.
    pub angle: A,
    /// Length.
    pub length: L,
    /// Mass.
    pub mass: M,
    /// Time.
    pub time: T,
    /// Electric Current.
    pub current: I,
    /// Thermodynamic Temperature.
    pub temperature: O,
    /// Luminous intensity.
    pub intensity: J,
    /// Amount of substance.
    pub amount: N,
}

impl<A, L, M, T, I, O, J, N> SiDimensions<A, L, M, T, I, O, J, N>
where
    A: AngleDimension<Exponent: Exponent<Value = i32>>,
    L: LengthDimension<Exponent: Exponent<Value = i32>>,
    M: MassDimension<Exponent: Exponent<Value = i32>>,
    T: TimeDimension<Exponent: Exponent<Value = i32>>,
    I: CurrentDimension<Exponent: Exponent<Value = i32>>,
    O: TemperatureDimension<Exponent: Exponent<Value = i32>>,
    J: LuminousIntensityDimension<Exponent: Exponent<Value = i32>>,
    N: AmountOfSubstanceDimension<Exponent: Exponent<Value = i32>>,
{
    fn format(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        format_dimensions(
            &[
                self.angle.exponent().value(),
                self.time.exponent().value(),
                self.length.exponent().value(),
                self.mass.exponent().value(),
                self.current.exponent().value(),
                self.temperature.exponent().value(),
                self.amount.exponent().value(),
                self.intensity.exponent().value(),
            ],
            &[
                &self.angle.symbol() as &dyn Symbol,
                &self.time.symbol(),
                &self.length.symbol(),
                &self.mass.symbol(),
                &self.current.symbol(),
                &self.temperature.symbol(),
                &self.amount.symbol(),
                &self.intensity.symbol(),
            ],
            f,
        )
    }
}

impl<A, L, M, T, I, O, J, N> fmt::Display for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: AngleDimension<Exponent: Exponent<Value = i32>>,
    L: LengthDimension<Exponent: Exponent<Value = i32>>,
    M: MassDimension<Exponent: Exponent<Value = i32>>,
    T: TimeDimension<Exponent: Exponent<Value = i32>>,
    I: CurrentDimension<Exponent: Exponent<Value = i32>>,
    O: TemperatureDimension<Exponent: Exponent<Value = i32>>,
    J: LuminousIntensityDimension<Exponent: Exponent<Value = i32>>,
    N: AmountOfSubstanceDimension<Exponent: Exponent<Value = i32>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.format(f)
    }
}

//
//  Arithmetic: addition/subtraction.
//

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> AddAssign<SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: AddAssign<OA>,
    L: AddAssign<OL>,
    M: AddAssign<OM>,
    T: AddAssign<OT>,
    I: AddAssign<OI>,
    O: AddAssign<OO>,
    J: AddAssign<OJ>,
    N: AddAssign<ON>,
{
    fn add_assign(&mut self, other: SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>) {
        self.angle += other.angle;
        self.length += other.length;
        self.mass += other.mass;
        self.time += other.time;
        self.current += other.current;
        self.temperature += other.temperature;
        self.intensity += other.intensity;
        self.amount += other.amount;
    }
}

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Add<SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: Add<OA>,
    L: Add<OL>,
    M: Add<OM>,
    T: Add<OT>,
    I: Add<OI>,
    O: Add<OO>,
    J: Add<OJ>,
    N: Add<ON>,
{
    type Output = SiDimensions<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn add(self, other: SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle + other.angle;
        let length = self.length + other.length;
        let mass = self.mass + other.mass;
        let time = self.time + other.time;
        let current = self.current + other.current;
        let temperature = self.temperature + other.temperature;
        let intensity = self.intensity + other.intensity;
        let amount = self.amount + other.amount;

        SiDimensions {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
        }
    }
}

impl<A, L, M, T, I, O, J, N> Neg for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: Neg,
    L: Neg,
    M: Neg,
    T: Neg,
    I: Neg,
    O: Neg,
    J: Neg,
    N: Neg,
{
    type Output = SiDimensions<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn neg(self) -> Self::Output {
        let angle = -self.angle;
        let length = -self.length;
        let mass = -self.mass;
        let time = -self.time;
        let current = -self.current;
        let temperature = -self.temperature;
        let intensity = -self.intensity;
        let amount = -self.amount;

        SiDimensions {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
        }
    }
}

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> SubAssign<SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: SubAssign<OA>,
    L: SubAssign<OL>,
    M: SubAssign<OM>,
    T: SubAssign<OT>,
    I: SubAssign<OI>,
    O: SubAssign<OO>,
    J: SubAssign<OJ>,
    N: SubAssign<ON>,
{
    fn sub_assign(&mut self, other: SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>) {
        self.angle -= other.angle;
        self.length -= other.length;
        self.mass -= other.mass;
        self.time -= other.time;
        self.current -= other.current;
        self.temperature -= other.temperature;
        self.intensity -= other.intensity;
        self.amount -= other.amount;
    }
}

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Sub<SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: Sub<OA>,
    L: Sub<OL>,
    M: Sub<OM>,
    T: Sub<OT>,
    I: Sub<OI>,
    O: Sub<OO>,
    J: Sub<OJ>,
    N: Sub<ON>,
{
    type Output = SiDimensions<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn sub(self, other: SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle - other.angle;
        let length = self.length - other.length;
        let mass = self.mass - other.mass;
        let time = self.time - other.time;
        let current = self.current - other.current;
        let temperature = self.temperature - other.temperature;
        let intensity = self.intensity - other.intensity;
        let amount = self.amount - other.amount;

        SiDimensions {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
        }
    }
}

//
//  Arithmetic: multiplication/division.
//

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Mul<SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: Mul<OA>,
    L: Mul<OL>,
    M: Mul<OM>,
    T: Mul<OT>,
    I: Mul<OI>,
    O: Mul<OO>,
    J: Mul<OJ>,
    N: Mul<ON>,
{
    type Output = SiDimensions<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn mul(self, other: SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle * other.angle;
        let length = self.length * other.length;
        let mass = self.mass * other.mass;
        let time = self.time * other.time;
        let current = self.current * other.current;
        let temperature = self.temperature * other.temperature;
        let intensity = self.intensity * other.intensity;
        let amount = self.amount * other.amount;

        SiDimensions {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
        }
    }
}

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Div<SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiDimensions<A, L, M, T, I, O, J, N>
where
    A: Div<OA>,
    L: Div<OL>,
    M: Div<OM>,
    T: Div<OT>,
    I: Div<OI>,
    O: Div<OO>,
    J: Div<OJ>,
    N: Div<ON>,
{
    type Output = SiDimensions<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn div(self, other: SiDimensions<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle / other.angle;
        let length = self.length / other.length;
        let mass = self.mass / other.mass;
        let time = self.time / other.time;
        let current = self.current / other.current;
        let temperature = self.temperature / other.temperature;
        let intensity = self.intensity / other.intensity;
        let amount = self.amount / other.amount;

        SiDimensions {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
        }
    }
}
