use crate::parser::Parser;
use std::marker::PhantomData;

pub struct Or<'i, T, P1, P2>(pub P1, pub P2, pub PhantomData<(&'i T, &'i T)>);

impl<'i, T, P1, P2> Parser<'i, T> for Or<'i, T, P1, P2>
where
    P1: Parser<'i, T>,
    P2: Parser<'i, T>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        self.0.parse(input).or_else(|| self.1.parse(input))
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.0
            .parse_discard(input)
            .or_else(|| self.1.parse_discard(input))
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        self.0.can_parse(input) || self.1.can_parse(input)
    }
}

impl<'i, T, P1, P2> Clone for Or<'i, T, P1, P2>
where
    P1: Clone,
    P2: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), Default::default())
    }
}

impl<'i, T, P1, P2> Copy for Or<'i, T, P1, P2>
where
    P1: Copy,
    P2: Copy,
{
}
