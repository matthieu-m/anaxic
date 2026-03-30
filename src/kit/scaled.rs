//! Collection of "scales".

use core::ops::{Add, AddAssign, Div, Mul, Neg, Sub, SubAssign};

use zahl::{Pow, RootOr, Z};

use crate::{
    api::{Dimension, Symbol, Unit},
    kit::{pow::Ten, symbol::CompositeSymbol},
};

//
//  Scaled unit basis.
//

/// A scaled unit, define by its scale (such as `Ten<3>` for 10³) and an anchor unit.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Scaled<S, A> {
    scale: S,
    anchor: A,
}

impl<S, A> Scaled<S, A> {
    /// Constructs a new instance.
    pub const fn from_parts(scale: S, anchor: A) -> Self {
        Self { scale, anchor }
    }

    /// Returns the parts.
    pub fn into_parts(self) -> (S, A) {
        (self.scale, self.anchor)
    }
}

//
//  Arithmetic: addition/subtraction.
//

impl<S, A, OS, OA> AddAssign<Scaled<OS, OA>> for Scaled<S, A>
where
    S: AddAssign<OS>,
    A: AddAssign<OA>,
{
    fn add_assign(&mut self, other: Scaled<OS, OA>) {
        self.scale += other.scale;
        self.anchor += other.anchor;
    }
}

impl<S, A, OS, OA> Add<Scaled<OS, OA>> for Scaled<S, A>
where
    S: Add<OS>,
    A: Add<OA>,
{
    type Output = Scaled<S::Output, A::Output>;

    fn add(self, other: Scaled<OS, OA>) -> Self::Output {
        let scale = self.scale + other.scale;
        let anchor = self.anchor + other.anchor;

        Scaled { scale, anchor }
    }
}

impl<S, A> Neg for Scaled<S, A>
where
    S: Neg,
    A: Neg,
{
    type Output = Scaled<S::Output, A::Output>;

    fn neg(self) -> Self::Output {
        let scale = -self.scale;
        let anchor = -self.anchor;

        Scaled { scale, anchor }
    }
}

impl<S, A, OS, OA> SubAssign<Scaled<OS, OA>> for Scaled<S, A>
where
    S: SubAssign<OS>,
    A: SubAssign<OA>,
{
    fn sub_assign(&mut self, other: Scaled<OS, OA>) {
        self.scale -= other.scale;
        self.anchor -= other.anchor;
    }
}

impl<S, A, OS, OA> Sub<Scaled<OS, OA>> for Scaled<S, A>
where
    S: Sub<OS>,
    A: Sub<OA>,
{
    type Output = Scaled<S::Output, A::Output>;

    fn sub(self, other: Scaled<OS, OA>) -> Self::Output {
        let scale = self.scale - other.scale;
        let anchor = self.anchor - other.anchor;

        Scaled { scale, anchor }
    }
}

//
//  Arithmetic: multiplication/division.
//

impl<S, A, OS, OA> Mul<Scaled<OS, OA>> for Scaled<S, A>
where
    S: Pow<A::Exponent>,
    OS: Pow<OA::Exponent>,
    <S as Pow<A::Exponent>>::Output: Mul<<OS as Pow<OA::Exponent>>::Output>,
    <<S as Pow<A::Exponent>>::Output as Mul<<OS as Pow<OA::Exponent>>::Output>>::Output:
        RootOr<<<A as Mul<OA>>::Output as Unit>::Exponent, Z<0>>,
    A: Mul<OA> + Unit,
    OA: Unit,
    A::Output: Unit,
{
    type Output = Scaled<
        <<<S as Pow<A::Exponent>>::Output as Mul<<OS as Pow<OA::Exponent>>::Output>>::Output as RootOr<
            <<A as Mul<OA>>::Output as Unit>::Exponent,
            Z<0>,
        >>::Output,
        A::Output,
    >;

    fn mul(self, other: Scaled<OS, OA>) -> Self::Output {
        let scale = self.scale.pow(self.anchor.exponent()) * other.scale.pow(other.anchor.exponent());
        let anchor = self.anchor * other.anchor;

        let scale = scale.root_or(anchor.exponent(), Z);

        Scaled { scale, anchor }
    }
}

impl<S, A, OS, OA> Div<Scaled<OS, OA>> for Scaled<S, A>
where
    S: Pow<A::Exponent>,
    OS: Pow<OA::Exponent>,
    <S as Pow<A::Exponent>>::Output: Div<<OS as Pow<OA::Exponent>>::Output>,
    <<S as Pow<A::Exponent>>::Output as Div<<OS as Pow<OA::Exponent>>::Output>>::Output:
        RootOr<<<A as Div<OA>>::Output as Unit>::Exponent, Z<0>>,
    A: Div<OA> + Unit,
    OA: Unit,
    A::Output: Unit,
{
    type Output = Scaled<
        <<<S as Pow<A::Exponent>>::Output as Div<<OS as Pow<OA::Exponent>>::Output>>::Output as RootOr<
            <<A as Div<OA>>::Output as Unit>::Exponent,
            Z<0>,
        >>::Output,
        A::Output,
    >;

    fn div(self, other: Scaled<OS, OA>) -> Self::Output {
        let scale = self.scale.pow(self.anchor.exponent()) / other.scale.pow(other.anchor.exponent());
        let anchor = self.anchor / other.anchor;

        let scale = scale.root_or(anchor.exponent(), Z);

        Scaled { scale, anchor }
    }
}

//
//  Unit.
//

impl<S, A> Unit for Scaled<S, A>
where
    S: ScalePrefix,
    A: Unit,
{
    type Dimension = A::Dimension;
    type Exponent = <A::Dimension as Dimension>::Exponent;
    type Symbol = CompositeSymbol<S::Prefix, A::Symbol>;

    fn dimension(&self) -> &Self::Dimension {
        self.anchor.dimension()
    }

    fn symbol(&self) -> Self::Symbol {
        CompositeSymbol::new(self.scale.prefix(), self.anchor.symbol())
    }
}

/// A prefix bundling both a scale and a prefix to apply to the unit.
pub trait ScalePrefix {
    /// The prefix of the scale.
    type Prefix: Symbol + 'static;

    /// The prefix of the scale.
    fn prefix(&self) -> Self::Prefix;
}

//
//  SI prefixes.
//

/// Quetta: 10³⁰.
pub type Quetta<A> = Scaled<Ten<30>, A>;

/// Ronna: 10²⁷.
pub type Ronna<A> = Scaled<Ten<27>, A>;

/// Yotta: 10²⁴.
pub type Yotta<A> = Scaled<Ten<24>, A>;

/// Zetta: 10²¹.
pub type Zetta<A> = Scaled<Ten<21>, A>;

/// Exa: 10¹⁸.
pub type Exa<A> = Scaled<Ten<18>, A>;

/// Peta: 10¹⁵.
pub type Peta<A> = Scaled<Ten<15>, A>;

/// Tera: 10¹².
pub type Tera<A> = Scaled<Ten<12>, A>;

/// Giga: 10⁹.
pub type Giga<A> = Scaled<Ten<9>, A>;

/// Mega: 10⁶.
pub type Mega<A> = Scaled<Ten<6>, A>;

/// Kilo: 10³.
pub type Kilo<A> = Scaled<Ten<3>, A>;

/// Hecto: 10².
pub type Hecto<A> = Scaled<Ten<2>, A>;

/// Deca: 10¹
pub type Deca<A> = Scaled<Ten<1>, A>;

/// No scale.
pub type Unscaled<A> = Scaled<Ten<0>, A>;

/// Deci: 10⁻¹
pub type Deci<A> = Scaled<Ten<-1>, A>;

/// Centi: 10⁻².
pub type Centi<A> = Scaled<Ten<-2>, A>;

/// Milli: 10⁻³.
pub type Milli<A> = Scaled<Ten<-3>, A>;

/// Micro: 10⁻⁶.
pub type Micro<A> = Scaled<Ten<-6>, A>;

/// Nano: 10⁻⁹.
pub type Nano<A> = Scaled<Ten<-9>, A>;

/// Pico: 10⁻¹².
pub type Pico<A> = Scaled<Ten<-12>, A>;

/// Femto: 10⁻¹⁵.
pub type Femto<A> = Scaled<Ten<-15>, A>;

/// Atto: 10⁻¹⁸.
pub type Atto<A> = Scaled<Ten<-18>, A>;

/// Zepto: 10⁻²¹.
pub type Zepto<A> = Scaled<Ten<-21>, A>;

/// Yocto: 10⁻²⁴.
pub type Yocto<A> = Scaled<Ten<-24>, A>;

/// Ronto: 10⁻²⁷.
pub type Ronto<A> = Scaled<Ten<-27>, A>;

/// Quecto: 10⁻³⁰.
pub type Quecto<A> = Scaled<Ten<-30>, A>;

macro_rules! impl_scale_prefix {
    ($scale:ty => $prefix:literal) => {
        impl ScalePrefix for $scale {
            type Prefix = &'static str;

            fn prefix(&self) -> Self::Prefix {
                $prefix
            }
        }
    };
}

impl_scale_prefix!(Ten<30> => "Q");
impl_scale_prefix!(Ten<27> => "R");
impl_scale_prefix!(Ten<24> => "Y");
impl_scale_prefix!(Ten<21> => "Z");
impl_scale_prefix!(Ten<18> => "E");
impl_scale_prefix!(Ten<15> => "P");
impl_scale_prefix!(Ten<12> => "T");
impl_scale_prefix!(Ten<9> => "G");
impl_scale_prefix!(Ten<6> => "M");
impl_scale_prefix!(Ten<3> => "k");
impl_scale_prefix!(Ten<2> => "h");
impl_scale_prefix!(Ten<1> => "da");
impl_scale_prefix!(Ten<0> => "");
impl_scale_prefix!(Ten<-1> => "d");
impl_scale_prefix!(Ten<-2> => "c");
impl_scale_prefix!(Ten<-3> => "m");
impl_scale_prefix!(Ten<-6> => "μ");
impl_scale_prefix!(Ten<-9> => "n");
impl_scale_prefix!(Ten<-12> => "p");
impl_scale_prefix!(Ten<-15> => "f");
impl_scale_prefix!(Ten<-18> => "a");
impl_scale_prefix!(Ten<-21> => "z");
impl_scale_prefix!(Ten<-24> => "y");
impl_scale_prefix!(Ten<-27> => "r");
impl_scale_prefix!(Ten<-30> => "q");

#[cfg(test)]
mod tests {
    use crate::{api::Dimension, kit::anchor::MeterAnchor};

    use super::*;

    type Kilometer = Kilo<MeterAnchor<1>>;
    type Meter = Unscaled<MeterAnchor<1>>;
    type Millimeter = Milli<MeterAnchor<1>>;

    #[test]
    fn arithmetic() {
        let kilometer = Kilometer::default();
        let meter = Meter::default();
        let millimeter = Millimeter::default();

        assert_unscaled(-meter, 1);
        assert_unscaled(meter + meter, 1);
        assert_unscaled(meter - meter, 1);

        assert_unscaled(meter * meter, 2);
        assert_unscaled(meter / meter, 0);
        assert_unscaled(meter / (meter * meter), -1);

        assert_kilo(-kilometer, 1);
        assert_kilo(kilometer + kilometer, 1);
        assert_kilo(kilometer - kilometer, 1);

        assert_kilo(kilometer * kilometer, 2);
        assert_unscaled(kilometer / kilometer, 0);
        assert_kilo(kilometer / (kilometer * kilometer), -1);

        assert_unscaled(kilometer * millimeter, 2);
        assert_unscaled(meter / millimeter, 0);
    }

    #[track_caller]
    fn assert_kilo<A>(unit: Kilo<A>, expected: i32)
    where
        A: Unit<Exponent: Into<i32>>,
    {
        assert_dimension(unit, expected);
    }

    fn assert_unscaled<A>(unit: Unscaled<A>, expected: i32)
    where
        A: Unit<Exponent: Into<i32>>,
    {
        assert_dimension(unit, expected);
    }

    #[track_caller]
    fn assert_dimension<U>(unit: U, expected: i32)
    where
        U: Unit<Exponent: Into<i32>>,
    {
        assert_eq!(expected, unit.dimension().exponent().into());
    }
} // mod tests
