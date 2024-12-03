use std::marker::PhantomData;
use crate::parser::Parser;
use crate::parser::repeat::Repeat;
use crate::utils::GatherTarget;

pub struct Rewind<P, T> {
    parser: P,
    spooky_ghost: PhantomData<T>,
}

impl<P, T> Rewind<P, T> {
    pub fn new(parser: P) -> Self {
        Self{parser, spooky_ghost: Default::default()}
    }
}

impl<'i, P, T> Parser<'i, T> for Rewind<P, T> where P: Parser<'i, T> {
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        self.parser.parse_value(input).map(|res| (res, input))
    }

    fn parse_first(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        self.parser.parse_first(input).map(|(res, _)| (res, input))
    }

    fn parse_value(&self, input: &'i [u8]) -> Option<T> {
        self.parser.parse_value(input)
    }

    fn parse_full(&self, _: &'i [u8]) -> Option<T> {
        None
    }

    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.parser.parse_discard(input).map(|_| input)
    }

    fn parse_discard_first(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.parser.parse_discard_first(input).map(|_| input)
    }

    fn repeat<G>(self) -> Repeat<T, Self, G>
    where
        G: GatherTarget<T>,
    {
        panic!("Repeat not a Rewind")
    }
}