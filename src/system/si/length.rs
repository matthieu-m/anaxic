//! SI units of length.

//  Captilization matters, as Qm != qm.
#![allow(non_upper_case_globals)]

use crate::{
    kit::{anchor::MeterAnchor, pow::Ten, scaled::*, zero::Zero},
    system::si::SiUnits,
};

/// A pure length unit in the SI system.
pub type Length<L> = SiUnits<Zero, L, Zero, Zero, Zero, Zero, Zero, Zero>;

/// Quettameter (Qm): 10³⁰ meters.
pub type QuettaMeter<const N: i32 = 1> = Length<Quetta<MeterAnchor<N>>>;

/// Quettameter.
pub const Qm: QuettaMeter<1> = Length::length(Quetta::from_parts(Ten::new(), MeterAnchor::new()));

/// Ronnameter (Rm): 10²⁷ meters.
pub type RonnaMeter<const N: i32 = 1> = Length<Ronna<MeterAnchor<N>>>;

/// Ronnameter.
pub const Rm: RonnaMeter<1> = Length::length(Ronna::from_parts(Ten::new(), MeterAnchor::new()));

/// Yottameter (Ym): 10²⁴ meters.
pub type YottaMeter<const N: i32 = 1> = Length<Yotta<MeterAnchor<N>>>;

/// Ronnameter.
pub const Ym: YottaMeter<1> = Length::length(Yotta::from_parts(Ten::new(), MeterAnchor::new()));

/// Zettameter (Zm): 10²¹ meters.
pub type ZettaMeter<const N: i32 = 1> = Length<Zetta<MeterAnchor<N>>>;

/// Zettameter.
pub const Zm: ZettaMeter<1> = Length::length(Zetta::from_parts(Ten::new(), MeterAnchor::new()));

/// Exameter (Em): 10¹⁸ meters.
pub type ExaMeter<const N: i32 = 1> = Length<Exa<MeterAnchor<N>>>;

/// Exameter.
pub const Em: ExaMeter<1> = Length::length(Exa::from_parts(Ten::new(), MeterAnchor::new()));

/// Petameter (Pm): 10¹⁵ meters.
pub type PetaMeter<const N: i32 = 1> = Length<Peta<MeterAnchor<N>>>;

/// Petameter.
pub const Pm: PetaMeter<1> = Length::length(Peta::from_parts(Ten::new(), MeterAnchor::new()));

/// Terameter (Tm): 10¹² meters.
pub type TeraMeter<const N: i32 = 1> = Length<Tera<MeterAnchor<N>>>;

/// Terameter.
pub const Tm: TeraMeter<1> = Length::length(Tera::from_parts(Ten::new(), MeterAnchor::new()));

/// Gigameter (Gm): 10⁹ meters.
pub type GigaMeter<const N: i32 = 1> = Length<Giga<MeterAnchor<N>>>;

/// Gigameter.
pub const Gm: GigaMeter<1> = Length::length(Giga::from_parts(Ten::new(), MeterAnchor::new()));

/// Megameter (Mm): 10⁶ meters.
pub type MegaMeter<const N: i32 = 1> = Length<Mega<MeterAnchor<N>>>;

/// Megameter.
pub const Mm: MegaMeter<1> = Length::length(Mega::from_parts(Ten::new(), MeterAnchor::new()));

/// Kilometer (km): 10³ meters.
pub type KiloMeter<const N: i32 = 1> = Length<Kilo<MeterAnchor<N>>>;

/// Kilometer.
pub const km: KiloMeter<1> = Length::length(Kilo::from_parts(Ten::new(), MeterAnchor::new()));

/// Hectometer (hm): 10² meters.
pub type HectoMeter<const N: i32 = 1> = Length<Hecto<MeterAnchor<N>>>;

/// Hectometer.
pub const hm: HectoMeter<1> = Length::length(Hecto::from_parts(Ten::new(), MeterAnchor::new()));

/// Decameter (dam): 10¹ meters.
pub type DecaMeter<const N: i32 = 1> = Length<Deca<MeterAnchor<N>>>;

/// Decameter.
pub const dam: DecaMeter<1> = Length::length(Deca::from_parts(Ten::new(), MeterAnchor::new()));

/// Meter (m).
pub type Meter<const N: i32 = 1> = Length<Unscaled<MeterAnchor<N>>>;

/// Meter.
pub const m: Meter<1> = Length::length(Unscaled::from_parts(Ten::new(), MeterAnchor::new()));

/// Decimeter (dm): 10⁻¹ meters.
pub type DeciMeter<const N: i32 = 1> = Length<Deci<MeterAnchor<N>>>;

/// Decimeter.
pub const dm: DeciMeter<1> = Length::length(Deci::from_parts(Ten::new(), MeterAnchor::new()));

/// Centimeter (cm): 10⁻² meters.
pub type CentiMeter<const N: i32 = 1> = Length<Centi<MeterAnchor<N>>>;

/// Centimeter.
pub const cm: CentiMeter<1> = Length::length(Centi::from_parts(Ten::new(), MeterAnchor::new()));

/// Millimeter (mm): 10⁻³ meters.
pub type MilliMeter<const N: i32 = 1> = Length<Milli<MeterAnchor<N>>>;

/// Millimeter.
pub const mm: MilliMeter<1> = Length::length(Milli::from_parts(Ten::new(), MeterAnchor::new()));

/// Micrometer (μm): 10⁻⁶ meters.
pub type MicroMeter<const N: i32 = 1> = Length<Micro<MeterAnchor<N>>>;

/// Micrometer.
pub const um: MicroMeter<1> = Length::length(Micro::from_parts(Ten::new(), MeterAnchor::new()));

/// Nanometer (nm): 10⁻⁹ meters.
pub type NanoMeter<const N: i32 = 1> = Length<Nano<MeterAnchor<N>>>;

/// Nanometer.
pub const nm: NanoMeter<1> = Length::length(Nano::from_parts(Ten::new(), MeterAnchor::new()));

/// Picometer (pm): 10⁻¹² meters.
pub type PicoMeter<const N: i32 = 1> = Length<Pico<MeterAnchor<N>>>;

/// Picometer.
pub const pm: PicoMeter<1> = Length::length(Pico::from_parts(Ten::new(), MeterAnchor::new()));

/// Femtometer (fm): 10⁻¹⁵ meters.
pub type FemtoMeter<const N: i32 = 1> = Length<Femto<MeterAnchor<N>>>;

/// Femtometer.
pub const fm: FemtoMeter<1> = Length::length(Femto::from_parts(Ten::new(), MeterAnchor::new()));

/// Attometer (am): 10⁻¹⁸ meters.
pub type AttoMeter<const N: i32 = 1> = Length<Atto<MeterAnchor<N>>>;

/// Attometer.
pub const am: AttoMeter<1> = Length::length(Atto::from_parts(Ten::new(), MeterAnchor::new()));

/// Zeptometer (zm): 10⁻²¹ meters.
pub type ZeptoMeter<const N: i32 = 1> = Length<Zepto<MeterAnchor<N>>>;

/// Zeptometer.
pub const zm: ZeptoMeter<1> = Length::length(Zepto::from_parts(Ten::new(), MeterAnchor::new()));

/// Yoctometer (ym): 10⁻²⁴ meters.
pub type YoctoMeter<const N: i32 = 1> = Length<Yocto<MeterAnchor<N>>>;

/// Yoctometer.
pub const ym: YoctoMeter<1> = Length::length(Yocto::from_parts(Ten::new(), MeterAnchor::new()));

/// Rontometer (rm): 10⁻²⁷ meters.
pub type RontoMeter<const N: i32 = 1> = Length<Ronto<MeterAnchor<N>>>;

/// Rontometer.
pub const rm: RontoMeter<1> = Length::length(Ronto::from_parts(Ten::new(), MeterAnchor::new()));

/// Quectometer (qm): 10⁻³⁰ meters.
pub type QuectoMeter<const N: i32 = 1> = Length<Quecto<MeterAnchor<N>>>;

/// Quectometer.
pub const qm: QuectoMeter<1> = Length::length(Quecto::from_parts(Ten::new(), MeterAnchor::new()));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brush() {
        assert_millimeter2(mm * mm);
    }

    fn assert_millimeter2(_: MilliMeter<2>) {}
} // mod tests
