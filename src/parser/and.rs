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

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<((T1, T2), usize, &'i [u8])> {
        if let Some((v1, index, next)) = self.0.find_parsable(input) {
            if let Some((v2, next)) = self.1.parse(next) {
                return Some(((v1, v2), index, next));
            }
        }

        None
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

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(T1, usize, &'i [u8])> {
        if let Some((v1, index, next)) = self.0.find_parsable(input) {
            if let Some((_, next)) = self.1.parse(next) {
                return Some((v1, index, next));
            }
        }

        None
    }
}

pub struct AndSkip<'i, T1, T2, P1, P2>(pub P1, pub P2, pub PhantomData<(&'i T1, &'i T2)>);

impl<'i, T1, T2, P1, P2> Parser<'i, T1> for AndSkip<'i, T1, T2, P1, P2>
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(T1, &'i [u8])> {
        let (v1, next) = self.0.parse(input)?;
        if let Some((_, next)) = self.1.parse(next) {
            Some((v1, next))
        } else {
            Some((v1, next))
        }
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        let next = self.0.parse_discard(input)?;
        if let Some(next) = self.1.parse_discard(next) {
            Some(next)
        } else {
            Some(next)
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        self.0.can_parse(input)
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(T1, usize, &'i [u8])> {
        self.0.find_parsable(input)
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

pub struct AndInstead<'i, T1, T2, P1, P2>(pub P1, pub P2, pub PhantomData<(&'i T1, &'i T2)>);

impl<'i, T1, T2, P1, P2> Parser<'i, T2> for AndInstead<'i, T1, T2, P1, P2>
where
    P1: Parser<'i, T1>,
    P2: Parser<'i, T2>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(T2, &'i [u8])> {
        let next = self.0.parse_discard(input)?;
        let (v2, next) = self.1.parse(next)?;

        Some((v2, next))
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        let next = self.0.parse_discard(input)?;
        let next = self.1.parse_discard(next)?;

        Some(next)
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(T2, usize, &'i [u8])> {
        if let Some((_, index, next)) = self.0.find_parsable(input) {
            if let Some((v2, next)) = self.1.parse(next) {
                return Some((v2, index, next));
            }
        }

        None
    }
}

impl<'i, T1, T2, P1, P2> Clone for AndInstead<'i, T1, T2, P1, P2>
where
    P1: Clone,
    P2: Clone,
{
    fn clone(&self) -> Self {
        AndInstead(self.0.clone(), self.1.clone(), PhantomData)
    }
}

impl<'i, T1, T2, P1, P2> Copy for AndInstead<'i, T1, T2, P1, P2>
where
    P1: Copy,
    P2: Copy,
{
}
