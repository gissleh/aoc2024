use crate::parser::Parser;
use std::marker::PhantomData;

pub struct Map<P, F, TI, TO> {
    parser: P,
    func: F,
    spooky_ghost: PhantomData<(TI, TO)>,
}

impl<P, F, TI, TO> Map<P, F, TI, TO> {
    pub(crate) fn new(parser: P, func: F) -> Self {
        Self {
            parser,
            func,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, P, F, TI, TO> Parser<'i, TO> for Map<P, F, TI, TO>
where
    P: Parser<'i, TI>,
    F: Fn(TI) -> TO,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(TO, &'i [u8])> {
        self.parser
            .parse(input)
            .map(|(res, next)| ((self.func)(res), next))
    }

    #[inline]
    fn parse_first(&self, input: &'i [u8]) -> Option<(TO, &'i [u8])> {
        self.parser
            .parse_first(input)
            .map(|(res, next)| ((self.func)(res), next))
    }
}
