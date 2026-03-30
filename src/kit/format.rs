//! Formatting utilities.

use core::fmt;

use crate::api::Symbol;

/// A utility to format dimensions.
pub fn format_dimensions(
    exponents: &[i32],
    symbols: &[&dyn Symbol],
    f: &mut fmt::Formatter<'_>,
) -> Result<(), fmt::Error> {
    use core::fmt::Write;

    debug_assert_eq!(exponents.len(), symbols.len());

    let dimensions = exponents.iter().zip(symbols).filter(|(e, _)| **e != 0);

    for (index, (exponent, symbol)) in dimensions.enumerate() {
        if index > 0 {
            f.write_char(' ')?;
        }

        write!(f, "{symbol}")?;

        if *exponent == 1 {
            continue;
        }

        let mut superscripter = SuperscriptWriter::new(&mut *f);
        write!(&mut superscripter, "{exponent}")?;
    }

    Ok(())
}

/// Decorator to transform numbers to subscript on the fly.
///
/// Unknown characters are written as-is.
#[derive(Clone, Debug)]
pub struct SubscriptWriter<W>(InnerWriter<W>);

impl<W> SubscriptWriter<W> {
    /// Constructs a new instance.
    pub const fn new(writer: W) -> Self {
        Self(InnerWriter(writer))
    }

    /// Returns the inner writer.
    pub fn into_inner(self) -> W {
        self.0.0
    }
}

impl<W> fmt::Write for SubscriptWriter<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        const DIGIT_OFFSET: u32 = '₀' as u32 - '0' as u32;

        match c {
            '0'..='9' => self.0.write_u32(c as u32 + DIGIT_OFFSET),
            '+' => self.0.write_char('₊'),
            '-' => self.0.write_char('₋'),
            _ => self.0.write_char(c),
        }
    }
}

/// Decorator to transform numbers to superscript on the fly.
///
/// Unknown characters are written as-is.
#[derive(Clone, Debug)]
pub struct SuperscriptWriter<W>(InnerWriter<W>);

impl<W> SuperscriptWriter<W> {
    /// Constructs a new instance.
    pub const fn new(writer: W) -> Self {
        Self(InnerWriter(writer))
    }

    /// Returns the inner writer.
    pub fn into_inner(self) -> W {
        self.0.0
    }
}

impl<W> fmt::Write for SuperscriptWriter<W>
where
    W: fmt::Write,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for c in s.chars() {
            self.write_char(c)?;
        }

        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        const DIGIT_2_3_OFFSET: u32 = '²' as u32 - '2' as u32;
        const DIIGT_0_9_OFFSET: u32 = '⁰' as u32 - '0' as u32;

        match c {
            '1' => self.0.write_char('¹'),
            '2' | '3' => self.0.write_u32(c as u32 + DIGIT_2_3_OFFSET),
            '0'..='9' => self.0.write_u32(c as u32 + DIIGT_0_9_OFFSET),
            '+' => self.0.write_char('⁺'),
            '-' => self.0.write_char('⁻'),
            _ => self.0.write_char(c),
        }
    }
}

//
//  Implementation
//

#[derive(Clone, Debug)]
struct InnerWriter<W>(W);

impl<W> InnerWriter<W>
where
    W: fmt::Write,
{
    #[inline(always)]
    fn write_char(&mut self, c: char) -> Result<(), fmt::Error> {
        self.0.write_char(c)
    }

    #[inline(always)]
    fn write_u32(&mut self, c: u32) -> Result<(), fmt::Error> {
        let Some(c) = char::from_u32(c) else {
            return Err(fmt::Error);
        };

        self.0.write_char(c)
    }
}

#[cfg(test)]
mod format_tests {
    use super::*;

    #[test]
    fn none() {
        assert_formatted(&[], &[], "");
    }

    #[test]
    fn single() {
        assert_formatted(&[0], &[&"T"], "");

        assert_formatted(&[1], &[&"T"], "T");

        assert_formatted(&[-2], &[&"T"], "T⁻²");
    }

    #[test]
    fn multiple() {
        assert_formatted(&[0, 1, 2], &[&"T", &"L", &"M"], "L M²");

        assert_formatted(&[2, -1, 0], &[&"T", &"L", &"M"], "T² L⁻¹");

        assert_formatted(&[-2, 0, 1], &[&"T", &"L", &"M"], "T⁻² M");
    }

    #[track_caller]
    fn assert_formatted(exponents: &[i32], symbols: &[&dyn Symbol], expected: &str) {
        use core::fmt::{self, Write};

        struct Harness<'a> {
            exponents: &'a [i32],
            symbols: &'a [&'a dyn Symbol],
        }

        impl fmt::Display for Harness<'_> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
                format_dimensions(self.exponents, self.symbols, f)
            }
        }

        let mut result = String::new();

        write!(&mut result, "{}", Harness { exponents, symbols }).expect("success");

        assert_eq!(expected, result);
    }
} // mod format_tests

#[cfg(test)]
mod script_tests {
    use super::*;

    #[test]
    fn subscript() {
        assert_subscripted(i32::MAX, "₂₁₄₇₄₈₃₆₄₇");
        assert_subscripted(i32::MIN, "₋₂₁₄₇₄₈₃₆₄₈");

        for i in 0..=9 {
            let c = char::from_u32('₀' as u32 + i as u32).expect("valid char");
            let s = format!("{c}");

            assert_subscripted(i, &s);
        }
    }

    #[track_caller]
    fn assert_subscripted(n: i32, expected: &str) {
        use std::fmt::Write;

        let mut writer = SubscriptWriter::new(String::new());

        write!(&mut writer, "{n}").expect("infallible");

        assert_eq!(expected, writer.0.0);
    }

    #[test]
    fn supercript() {
        assert_superscripted(i32::MAX, "²¹⁴⁷⁴⁸³⁶⁴⁷");
        assert_superscripted(i32::MIN, "⁻²¹⁴⁷⁴⁸³⁶⁴⁸");

        assert_superscripted(0, "⁰");
        assert_superscripted(1, "¹");
        assert_superscripted(2, "²");
        assert_superscripted(3, "³");

        for i in 4..=9 {
            let c = char::from_u32('⁰' as u32 + i as u32).expect("valid char");
            let s = format!("{c}");

            assert_superscripted(i, &s);
        }
    }

    #[track_caller]
    fn assert_superscripted(n: i32, expected: &str) {
        use std::fmt::Write;

        let mut writer = SuperscriptWriter::new(String::new());

        write!(&mut writer, "{n}").expect("infallible");

        assert_eq!(expected, writer.0.0);
    }
} // mod script_tests
