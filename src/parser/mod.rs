mod and;
mod basic;
mod conditional;
mod delimiter;
mod extract;
mod map;
mod numbers;
mod or;
mod repeat;
mod rewind;
mod within;

use crate::parser::and::{AndInstead, AndSkip};
use crate::parser::delimiter::DelimitedBy;
use crate::parser::extract::Extract;
use crate::parser::map::Map;
use crate::parser::or::Or;
use crate::parser::repeat::{Repeat, RepeatFold, RepeatFoldMut};
use crate::parser::rewind::Rewind;
use crate::parser::within::{QuotedBy, Within};
use crate::utils::GatherTarget;
pub use and::{And, AndDiscard};
pub use basic::{everything, line, n_bytes, word, word_terminated_by};
pub use conditional::OnlyIf;
pub use numbers::{base62_digit, digit, hex_digit, int, signed_int, uint, unsigned_int};

pub trait Parser<'i, T>: Sized {
    /// The main parsing function.
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])>;

    /// Parse, but do special behavior if it is the first in a series.
    #[inline]
    fn parse_first(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        self.parse(input)
    }

    /// Parse and drop the remaining input.
    #[inline]
    fn parse_value(&self, input: &'i [u8]) -> Option<T> {
        self.parse(input).map(|(value, _)| value)
    }

    /// Parse and return only if that consumed the whole input.
    #[inline]
    fn parse_full(&self, input: &'i [u8]) -> Option<T> {
        match self.parse(input) {
            Some((value, next)) if next.is_empty() => Some(value),
            _ => None,
        }
    }

    /// Parse and discard the output. Parsers may implement this for optimization.
    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.parse(input).map(|(_, next)| next)
    }

    /// Parse and discard the output, but apply first-specific behavior.
    #[inline]
    fn parse_discard_first(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.parse_discard(input)
    }

    /// Check if it can be parsed. If parse_discard is optimized, then overriding this one
    /// is probably a waste of time.
    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        self.parse_discard(input).is_some()
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(T, usize, &'i [u8])> {
        for index in 0..input.len() {
            if let Some((res, next)) = self.parse(&input[index..]) {
                return Some((res, index, next));
            }
        }

        None
    }

    #[inline]
    fn and<T2, P2>(self, rhs: P2) -> And<'i, T, T2, Self, P2>
    where
        P2: Parser<'i, T2>,
    {
        And(self, rhs, Default::default())
    }

    #[inline]
    fn and_discard<T2, P2>(self, rhs: P2) -> AndDiscard<'i, T, T2, Self, P2>
    where
        P2: Parser<'i, T2>,
    {
        AndDiscard(self, rhs, Default::default())
    }

    #[inline]
    fn and_skip<T2, P2>(self, rhs: P2) -> AndSkip<'i, T, T2, Self, P2>
    where
        P2: Parser<'i, T2>,
    {
        AndSkip(self, rhs, Default::default())
    }

    #[inline]
    fn and_instead<T2, P2>(self, rhs: P2) -> AndInstead<'i, T, T2, Self, P2>
    where
        P2: Parser<'i, T2>,
    {
        AndInstead(self, rhs, Default::default())
    }

    #[inline]
    fn or<P2>(self, rhs: P2) -> Or<'i, T, Self, P2>
    where
        P2: Parser<'i, T>,
    {
        Or(self, rhs, Default::default())
    }

    /// Dis
    #[inline]
    fn only_if<F>(self, cb: F) -> OnlyIf<'i, Self, T, F>
    where
        F: Fn(&T) -> bool,
    {
        OnlyIf(self, cb, Default::default())
    }

    /// Map the result of the parser into a new type.
    #[inline]
    fn map<F, TO>(self, f: F) -> Map<Self, F, T, TO> {
        Map::new(self, f)
    }

    #[inline]
    fn repeat<G>(self) -> Repeat<T, Self, G>
    where
        G: GatherTarget<T>,
    {
        self.repeat_limited(0, 0)
    }

    /// Repeat with limitations.
    #[inline]
    fn repeat_limited<G>(self, min: usize, max: usize) -> Repeat<T, Self, G>
    where
        G: GatherTarget<T>,
    {
        Repeat::new(self, min, max)
    }

    /// Repeat with a fold-style callback
    #[inline]
    fn repeat_fold<TO, FI, FF>(self, init_f: FI, fold_f: FF) -> RepeatFold<T, TO, Self, FI, FF>
    where
        FI: Fn() -> TO,
        FF: Fn(TO, T) -> TO,
    {
        RepeatFold::new(self, init_f, fold_f)
    }

    /// Repeat with a fold-like callback, except it borrows the object instead of moving it into the callback.
    /// Useful for when the fold state is massive.
    #[inline]
    fn repeat_fold_mut<TO, FI, FF>(
        self,
        init_f: FI,
        fold_f: FF,
    ) -> RepeatFoldMut<T, TO, Self, FI, FF>
    where
        FI: Fn() -> TO,
        FF: Fn(&mut TO, T),
    {
        RepeatFoldMut::new(self, init_f, fold_f)
    }

    /// Parse within an outer parser, requires that the inner parser is exhausting.
    #[inline]
    fn within<PO>(self, outer_parser: PO) -> Within<Self, PO, T>
    where
        PO: Parser<'i, &'i [u8]>,
    {
        Within::new(self, outer_parser)
    }

    /// Returns a parser that checks for a delimiter on `parse`, but not on `parse_first`
    #[inline]
    fn delimited_by<PD, TD>(self, delim: PD) -> DelimitedBy<Self, PD, T, TD> {
        DelimitedBy::new(self, delim)
    }

    /// Returns a parser that parses, but does not advance the input. Does not allow repeating.
    #[inline]
    fn rewind(self) -> Rewind<Self, T> {
        Rewind::new(self)
    }

    #[inline]
    fn extract(self) -> Extract<Self, T> {
        Extract::new(self)
    }

    #[inline]
    fn quoted_by<PL, PR, TL, TR>(
        self,
        left_parser: PL,
        right_parser: PR,
    ) -> QuotedBy<Self, PL, PR, T, TL, TR>
    where
        PL: Parser<'i, TL>,
        PR: Parser<'i, TR>,
    {
        QuotedBy::new(self, left_parser, right_parser)
    }
}

impl<'i> Parser<'i, u8> for u8 {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(u8, &'i [u8])> {
        if !input.is_empty() && input[0].eq(self) {
            Some((*self, &input[1..]))
        } else {
            None
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        !input.is_empty() && input[0].eq(self)
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(u8, usize, &'i [u8])> {
        input
            .iter()
            .position(|c| *c == *self)
            .map(|index| (*self, index, &input[index + 1..]))
    }
}

impl<'i, const N: usize> Parser<'i, &'i [u8]> for &'static [u8; N] {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(&'i [u8], &'i [u8])> {
        if input.starts_with(*self) {
            Some((&input[..self.len()], &input[self.len()..]))
        } else {
            None
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        input.starts_with(self.as_slice())
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(&'i [u8], usize, &'i [u8])> {
        input
            .windows(self.len())
            .position(|w| w == self.as_slice())
            .map(|index| {
                (
                    &input[index..index + self.len()],
                    index,
                    &input[index + self.len()..],
                )
            })
    }
}

impl<'i> Parser<'i, &'i [u8]> for &'static [u8] {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(&'i [u8], &'i [u8])> {
        if input.starts_with(self) {
            Some((&input[..self.len()], &input[self.len()..]))
        } else {
            None
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        input.starts_with(self)
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(&'i [u8], usize, &'i [u8])> {
        input
            .windows(self.len())
            .position(|w| w == *self)
            .map(|index| {
                (
                    &input[index..index + self.len()],
                    index,
                    &input[index + self.len()..],
                )
            })
    }
}
