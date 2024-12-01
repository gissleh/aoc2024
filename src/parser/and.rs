use crate::parser::Parser;
use std::marker::PhantomData;

pub struct And<'i, T1, T2, P1, P2>(pub P1, pub P2, pub PhantomData<(&'i T1, &'i T2)>);

impl<'i, T1, T2, P1, P2> Parser<'i, (T1, T2)> for And<'i, T1, T2, P1, P2>
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<((T1, T2), &'i [u8])> {
        let (v1, next) = self.0.parse(input)?;
        let (v2, next) = self.1.parse(next)?;

        Some(((v1, v2), next))
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        let next = self.0.parse_discard(input)?;
        let next = self.1.parse_discard(next)?;

        Some(next)
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        if let Some(next) = self.0.parse_discard(input) {
            self.1.can_parse(next)
        } else {
            false
        }
    }
}

impl<'i, T1, T2, P1, P2> Clone for And<'i, T1, T2, P1, P2>
where
    P1: Clone,
    P2: Clone,
{
    fn clone(&self) -> Self {
        And(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<'i, T1, T2, P1, P2> Copy for And<'i, T1, T2, P1, P2>
where
    P1: Copy,
    P2: Copy,
{
}

pub struct AndDiscard<'i, T1, T2, P1, P2>(pub P1, pub P2, pub PhantomData<(&'i T1, &'i T2)>);

impl<'i, T1, T2, P1, P2> Parser<'i, T1> for AndDiscard<'i, T1, T2, P1, P2>
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(T1, &'i [u8])> {
        let (v1, next) = self.0.parse(input)?;
        let next = self.1.parse_discard(next)?;

        Some((v1, next))
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        let next = self.0.parse_discard(input)?;
        let next = self.1.parse_discard(next)?;

        Some(next)
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        if let Some(next) = self.0.parse_discard(input) {
            self.1.can_parse(next)
        } else {
            false
        }
    }
}

impl<'i, T1, T2, P1, P2> Clone for AndDiscard<'i, T1, T2, P1, P2>
where
    P1: Clone,
    P2: Clone,
{
    fn clone(&self) -> Self {
        AndDiscard(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<'i, T1, T2, P1, P2> Copy for AndDiscard<'i, T1, T2, P1, P2>
where
    P1: Copy,
    P2: Copy,
{
}
