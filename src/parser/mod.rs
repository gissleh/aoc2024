mod and;
mod basic;
mod conditional;
mod numbers;
mod repeat;
mod within;

use crate::parser::repeat::{Repeat, RepeatFold};
use crate::parser::within::{QuotedBy, Within};
use crate::utils::GatherTarget;
pub use and::{And, AndDiscard};
pub use basic::{everything, line, word};
pub use conditional::OnlyIf;
pub use numbers::{digit, hex_digit, int, signed_int, uint, unsigned_int};

pub trait Parser<'i, T>: Sized {
    /// The main parsing function.
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])>;

    /// The main parsing function.
    #[inline]
    fn parse_value(&self, input: &'i [u8]) -> Option<T> {
        self.parse(input).map(|(value, _)| value)
    }

    /// Parse and discard the output. Parsers may implement this for optimization.
    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.parse(input).map(|(_, next)| next)
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
    fn only_if<F>(self, cb: F) -> OnlyIf<'i, Self, T, F>
    where
        F: Fn(&T) -> bool,
    {
        OnlyIf(self, cb, Default::default())
    }

    #[inline]
    fn repeat<G>(self) -> Repeat<T, Self, G>
    where
        G: GatherTarget<T>,
    {
        self.repeat_limited(0, 0)
    }

    #[inline]
    fn repeat_limited<G>(self, min: usize, max: usize) -> Repeat<T, Self, G>
    where
        G: GatherTarget<T>,
    {
        Repeat::new(self, min, max)
    }

    #[inline]
    fn repeat_fold<TO, FI, FF>(self, init_f: FI, fold_f: FF) -> RepeatFold<T, TO, Self, FI, FF>
    where
        FI: Fn() -> TO,
        FF: Fn(TO, T) -> TO,
    {
        RepeatFold::new(self, init_f, fold_f)
    }

    #[inline]
    fn within<PO>(self, outer_parser: PO) -> Within<Self, PO, T>
    where
        PO: Parser<'i, &'i [u8]>,
    {
        Within::new(self, outer_parser)
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

impl<'i, const N: usize> Parser<'i, &'i [u8]> for [u8; N] {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(&'i [u8], &'i [u8])> {
        if input.starts_with(self.as_slice()) {
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
