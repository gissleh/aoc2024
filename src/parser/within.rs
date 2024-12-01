use crate::parser::Parser;
use std::marker::PhantomData;

pub struct Within<PI, PO, T> {
    inner_parser: PI,
    outer_parser: PO,
    spooky_ghost: PhantomData<T>,
}

impl<PI, PO, T> Within<PI, PO, T> {
    pub fn new(inner_parser: PI, outer_parser: PO) -> Self {
        Self {
            inner_parser,
            outer_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, PO, T> Parser<'i, T> for Within<PI, PO, T>
where
    PI: Parser<'i, T>,
    PO: Parser<'i, &'i [u8]>,
{
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        if let Some((inner_input, next)) = self.outer_parser.parse(input) {
            if let Some((value, inner_next)) = self.inner_parser.parse(&inner_input) {
                if inner_next.is_empty() {
                    return Some((value, next));
                }
            }
        }

        None
    }
}

pub struct QuotedBy<PI, PL, PR, TI, TL, TR> {
    inner_parser: PI,
    left_parser: PL,
    right_parser: PR,
    spooky_ghost: PhantomData<(TI, TL, TR)>,
}

impl<PI, PL, PR, TI, TL, TR> QuotedBy<PI, PL, PR, TI, TL, TR> {
    pub fn new(inner_parser: PI, left_parser: PL, right_parser: PR) -> Self {
        Self {
            inner_parser,
            left_parser,
            right_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PI, PL, PR, TI, TL, TR> Parser<'i, TI> for QuotedBy<PI, PL, PR, TI, TL, TR>
where
    PI: Parser<'i, TI>,
    PL: Parser<'i, TL>,
    PR: Parser<'i, TR>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(TI, &'i [u8])> {
        if let Some(input) = self.left_parser.parse_discard(input) {
            if let Some((_, index, next)) = self.right_parser.find_parsable(input) {
                if let Some((res, remainder)) = self.inner_parser.parse(&input[..index]) {
                    if remainder.is_empty() {
                        return Some((res, next));
                    }
                }
            }
        }

        None
    }
}
