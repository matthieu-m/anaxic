//! SI units.

use core::{
    fmt,
    ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign},
};

use crate::{
    api::{
        Exponent, ScaleTo, ScaleToWith, Symbol, TransformPointTo, TransformPointToWith, TransformVectorTo,
        TransformVectorToWith, Unit,
    },
    kit::{
        dimension::{
            AmountOfSubstanceDimension, AngleDimension, CurrentDimension, LengthDimension, LuminousIntensityDimension,
            MassDimension, TemperatureDimension, TimeDimension,
        },
        format::format_dimensions,
        system::System,
        zero::Zero,
    },
};

/// Units of the SI system.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SiUnits<A, L, M, T, I, O, J, N> {
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

impl SiUnits<Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero> {
    /// Constructs a zero unit.
    pub const fn zero() -> Self {
        let (angle, length, mass, time, current, temperature, intensity, amount) = zeroes();

        Self {
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

impl<A> SiUnits<A, Zero, Zero, Zero, Zero, Zero, Zero, Zero> {
    /// Constructs a new angle.
    pub const fn angle(angle: A) -> Self {
        let SiUnits {
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<L> SiUnits<Zero, L, Zero, Zero, Zero, Zero, Zero, Zero> {
    /// Constructs a new length.
    pub const fn length(length: L) -> Self {
        let SiUnits {
            angle,
            mass,
            time,
            current,
            temperature,
            intensity,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<M> SiUnits<Zero, Zero, M, Zero, Zero, Zero, Zero, Zero> {
    /// Constructs a new mass.
    pub const fn mass(mass: M) -> Self {
        let SiUnits {
            angle,
            length,
            time,
            current,
            temperature,
            intensity,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<T> SiUnits<Zero, Zero, Zero, T, Zero, Zero, Zero, Zero> {
    /// Constructs a new time.
    pub const fn time(time: T) -> Self {
        let SiUnits {
            angle,
            length,
            mass,
            current,
            temperature,
            intensity,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<I> SiUnits<Zero, Zero, Zero, Zero, I, Zero, Zero, Zero> {
    /// Constructs a new current.
    pub const fn current(current: I) -> Self {
        let SiUnits {
            angle,
            length,
            mass,
            time,
            temperature,
            intensity,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<O> SiUnits<Zero, Zero, Zero, Zero, Zero, O, Zero, Zero> {
    /// Constructs a new temperature.
    pub const fn temperature(temperature: O) -> Self {
        let SiUnits {
            angle,
            length,
            mass,
            time,
            current,
            intensity,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<J> SiUnits<Zero, Zero, Zero, Zero, Zero, Zero, J, Zero> {
    /// Constructs a new intensity.
    pub const fn intensity(intensity: J) -> Self {
        let SiUnits {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            amount,
            ..
        } = SiUnits::zero();

        Self {
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

impl<N> SiUnits<Zero, Zero, Zero, Zero, Zero, Zero, Zero, N> {
    /// Constructs a new amount.
    pub const fn amount(amount: N) -> Self {
        let SiUnits {
            angle,
            length,
            mass,
            time,
            current,
            temperature,
            intensity,
            ..
        } = SiUnits::zero();

        Self {
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

impl<A, L, M, T, I, O, J, N> SiUnits<A, L, M, T, I, O, J, N>
where
    A: Unit<Dimension: AngleDimension<Exponent: Exponent<Value = i32>>>,
    L: Unit<Dimension: LengthDimension<Exponent: Exponent<Value = i32>>>,
    M: Unit<Dimension: MassDimension<Exponent: Exponent<Value = i32>>>,
    T: Unit<Dimension: TimeDimension<Exponent: Exponent<Value = i32>>>,
    I: Unit<Dimension: CurrentDimension<Exponent: Exponent<Value = i32>>>,
    O: Unit<Dimension: TemperatureDimension<Exponent: Exponent<Value = i32>>>,
    J: Unit<Dimension: LuminousIntensityDimension<Exponent: Exponent<Value = i32>>>,
    N: Unit<Dimension: AmountOfSubstanceDimension<Exponent: Exponent<Value = i32>>>,
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

impl<A, L, M, T, I, O, J, N> fmt::Display for SiUnits<A, L, M, T, I, O, J, N>
where
    A: Unit<Dimension: AngleDimension<Exponent: Exponent<Value = i32>>>,
    L: Unit<Dimension: LengthDimension<Exponent: Exponent<Value = i32>>>,
    M: Unit<Dimension: MassDimension<Exponent: Exponent<Value = i32>>>,
    T: Unit<Dimension: TimeDimension<Exponent: Exponent<Value = i32>>>,
    I: Unit<Dimension: CurrentDimension<Exponent: Exponent<Value = i32>>>,
    O: Unit<Dimension: TemperatureDimension<Exponent: Exponent<Value = i32>>>,
    J: Unit<Dimension: LuminousIntensityDimension<Exponent: Exponent<Value = i32>>>,
    N: Unit<Dimension: AmountOfSubstanceDimension<Exponent: Exponent<Value = i32>>>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.format(f)
    }
}

const fn zeroes() -> (Zero, Zero, Zero, Zero, Zero, Zero, Zero, Zero) {
    (
        Zero::new(),
        Zero::new(),
        Zero::new(),
        Zero::new(),
        Zero::new(),
        Zero::new(),
        Zero::new(),
        Zero::new(),
    )
}

//
//  Arithmetic: addition/subtraction.
//

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> AddAssign<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
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
    fn add_assign(&mut self, other: SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>) {
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

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Add<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
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
    type Output = SiUnits<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn add(self, other: SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle + other.angle;
        let length = self.length + other.length;
        let mass = self.mass + other.mass;
        let time = self.time + other.time;
        let current = self.current + other.current;
        let temperature = self.temperature + other.temperature;
        let intensity = self.intensity + other.intensity;
        let amount = self.amount + other.amount;

        SiUnits {
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

impl<A, L, M, T, I, O, J, N> Neg for SiUnits<A, L, M, T, I, O, J, N>
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
    type Output = SiUnits<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn neg(self) -> Self::Output {
        let angle = -self.angle;
        let length = -self.length;
        let mass = -self.mass;
        let time = -self.time;
        let current = -self.current;
        let temperature = -self.temperature;
        let intensity = -self.intensity;
        let amount = -self.amount;

        SiUnits {
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

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> SubAssign<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
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
    fn sub_assign(&mut self, other: SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>) {
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

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Sub<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
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
    type Output = SiUnits<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn sub(self, other: SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle - other.angle;
        let length = self.length - other.length;
        let mass = self.mass - other.mass;
        let time = self.time - other.time;
        let current = self.current - other.current;
        let temperature = self.temperature - other.temperature;
        let intensity = self.intensity - other.intensity;
        let amount = self.amount - other.amount;

        SiUnits {
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

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Mul<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
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
    type Output = SiUnits<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn mul(self, other: SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle * other.angle;
        let length = self.length * other.length;
        let mass = self.mass * other.mass;
        let time = self.time * other.time;
        let current = self.current * other.current;
        let temperature = self.temperature * other.temperature;
        let intensity = self.intensity * other.intensity;
        let amount = self.amount * other.amount;

        SiUnits {
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

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> Div<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
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
    type Output = SiUnits<A::Output, L::Output, M::Output, T::Output, I::Output, O::Output, J::Output, N::Output>;

    fn div(self, other: SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>) -> Self::Output {
        let angle = self.angle / other.angle;
        let length = self.length / other.length;
        let mass = self.mass / other.mass;
        let time = self.time / other.time;
        let current = self.current / other.current;
        let temperature = self.temperature / other.temperature;
        let intensity = self.intensity / other.intensity;
        let amount = self.amount / other.amount;

        SiUnits {
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
//  ScaleTo & ScaleToWith.
//

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON> ScaleTo<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>>
    for SiUnits<A, L, M, T, I, O, J, N>
where
    SiUnits<A, L, M, T, I, O, J, N>: Clone,
    System<(A, L, M, T, I, O, J, N)>: ScaleTo<System<(OA, OL, OM, OT, OI, OO, OJ, ON)>>,
{
    type Scale = <System<(A, L, M, T, I, O, J, N)> as ScaleTo<System<(OA, OL, OM, OT, OI, OO, OJ, ON)>>>::Scale;

    fn scale(&self) -> Self::Scale {
        self.clone().into_system().scale()
    }
}

impl<A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON, Context>
    ScaleToWith<SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>, Context> for SiUnits<A, L, M, T, I, O, J, N>
where
    SiUnits<A, L, M, T, I, O, J, N>: Clone,
    System<(A, L, M, T, I, O, J, N)>: ScaleToWith<System<(OA, OL, OM, OT, OI, OO, OJ, ON)>, Context>,
{
    type Scale =
        <System<(A, L, M, T, I, O, J, N)> as ScaleToWith<System<(OA, OL, OM, OT, OI, OO, OJ, ON)>, Context>>::Scale;

    fn scale_with(&self, context: &Context) -> Self::Scale {
        self.clone().into_system().scale_with(context)
    }
}

//
//  Transforms
//

impl<Q, A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON>
    TransformPointTo<Q, SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>> for SiUnits<A, L, M, T, I, O, J, N>
where
    SiUnits<A, L, M, T, I, O, J, N>: Clone,
    System<(A, L, M, T, I, O, J, N)>: TransformPointTo<Q, System<(OA, OL, OM, OT, OI, OO, OJ, ON)>>,
{
    type Output =
        <System<(A, L, M, T, I, O, J, N)> as TransformPointTo<Q, System<(OA, OL, OM, OT, OI, OO, OJ, ON)>>>::Output;

    fn transform_point(&self, value: Q) -> Self::Output {
        self.clone().into_system().transform_point(value)
    }
}

impl<Q, A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON>
    TransformVectorTo<Q, SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>> for SiUnits<A, L, M, T, I, O, J, N>
where
    SiUnits<A, L, M, T, I, O, J, N>: Clone,
    System<(A, L, M, T, I, O, J, N)>: TransformVectorTo<Q, System<(OA, OL, OM, OT, OI, OO, OJ, ON)>>,
{
    type Output =
        <System<(A, L, M, T, I, O, J, N)> as TransformVectorTo<Q, System<(OA, OL, OM, OT, OI, OO, OJ, ON)>>>::Output;

    fn transform_vector(&self, value: Q) -> Self::Output {
        self.clone().into_system().transform_vector(value)
    }
}

impl<Q, A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON, Context>
    TransformPointToWith<Q, SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>, Context> for SiUnits<A, L, M, T, I, O, J, N>
where
    SiUnits<A, L, M, T, I, O, J, N>: Clone,
    System<(A, L, M, T, I, O, J, N)>: TransformPointToWith<Q, System<(OA, OL, OM, OT, OI, OO, OJ, ON)>, Context>,
{
    type Output = <System<(A, L, M, T, I, O, J, N)> as TransformPointToWith<
        Q,
        System<(OA, OL, OM, OT, OI, OO, OJ, ON)>,
        Context,
    >>::Output;

    fn transform_point_with(&self, value: Q, context: &Context) -> Self::Output {
        self.clone().into_system().transform_point_with(value, context)
    }
}

impl<Q, A, L, M, T, I, O, J, N, OA, OL, OM, OT, OI, OO, OJ, ON, Context>
    TransformVectorToWith<Q, SiUnits<OA, OL, OM, OT, OI, OO, OJ, ON>, Context> for SiUnits<A, L, M, T, I, O, J, N>
where
    SiUnits<A, L, M, T, I, O, J, N>: Clone,
    System<(A, L, M, T, I, O, J, N)>: TransformVectorToWith<Q, System<(OA, OL, OM, OT, OI, OO, OJ, ON)>, Context>,
{
    type Output = <System<(A, L, M, T, I, O, J, N)> as TransformVectorToWith<
        Q,
        System<(OA, OL, OM, OT, OI, OO, OJ, ON)>,
        Context,
    >>::Output;

    fn transform_vector_with(&self, value: Q, context: &Context) -> Self::Output {
        self.clone().into_system().transform_vector_with(value, context)
    }
}

impl<A, L, M, T, I, O, J, N> SiUnits<A, L, M, T, I, O, J, N> {
    #[allow(clippy::type_complexity)]
    fn into_system(self) -> System<(A, L, M, T, I, O, J, N)> {
        System::from_tuple((
            self.angle,
            self.length,
            self.mass,
            self.time,
            self.current,
            self.temperature,
            self.intensity,
            self.amount,
        ))
    }
}
