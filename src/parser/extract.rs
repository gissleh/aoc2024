use crate::parser::Parser;
use std::marker::PhantomData;

pub struct Extract<P, T> {
    parser: P,
    spooky_ghost: PhantomData<T>,
}

impl<P, T> Extract<P, T> {
    #[inline]
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, P, T> Parser<'i, T> for Extract<P, T>
where
    P: Parser<'i, T>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        self.parser
            .find_parsable(input)
            .map(|(res, _, next)| (res, next))
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(T, usize, &'i [u8])> {
        self.parser.find_parsable(input)
    }
}
