use crate::parser::Parser;
use crate::utils::GatherTarget;
use std::marker::PhantomData;

pub struct Repeat<T, P, G> {
    parser: P,
    min: usize,
    max: usize,
    spooky_ghost: PhantomData<(G, T)>,
}

impl<T, P, G> Repeat<T, P, G> {
    pub fn new(parser: P, min: usize, max: usize) -> Self {
        Self {
            parser,
            min,
            max,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, T, P, G> Parser<'i, G> for Repeat<T, P, G>
where
    P: Parser<'i, T>,
    G: GatherTarget<T>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(G, &'i [u8])> {
        let mut target = G::init_gather_target(self.min);
        let mut input = input;

        if let Some((res, next)) = self.parser.parse_first(input) {
            if target.gather(0, res) {
                let mut index = 1usize;
                input = next;

                while let Some((res, next)) = self.parser.parse(input) {
                    input = next;
                    if !target.gather(index, res) {
                        break;
                    }
                    index += 1;
                    if index == self.max {
                        break;
                    }
                }

                if index < self.min {
                    return None;
                }
            } else if self.min > 0 {
                return None;
            }
        } else if self.min > 0 {
            return None;
        }

        Some((target, input))
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        if let Some((_, next)) = self.parser.parse_first(input) {
            let mut index = 1usize;
            let mut input = next;

            while let Some(next) = self.parser.parse_discard(input) {
                input = next;
                index += 1;
                if index == self.max {
                    break;
                }
            }

            if index < self.min {
                return None;
            }
        }

        if self.min != 0 {
            return None;
        }

        Some(input)
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        if self.min == 0 && self.max == 0 {
            true
        } else {
            self.parse_discard(input).is_some()
        }
    }
}

pub struct RepeatFold<TI, TO, P, FI, FF> {
    parser: P,
    init_f: FI,
    fold_f: FF,
    spooky_ghost: PhantomData<(TI, TO)>,
}

impl<TI, TO, P, FI, FF> RepeatFold<TI, TO, P, FI, FF> {
    pub fn new(parser: P, init_f: FI, fold_f: FF) -> Self {
        Self {
            parser,
            init_f,
            fold_f,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, TI, TO, P, FI, FF> Parser<'i, TO> for RepeatFold<TI, TO, P, FI, FF>
where
    P: Parser<'i, TI>,
    FI: Fn() -> TO,
    FF: Fn(TO, TI) -> TO,
{
    fn parse(&self, input: &'i [u8]) -> Option<(TO, &'i [u8])> {
        if let Some((res, input)) = self.parser.parse_first(input) {
            let mut state = (self.fold_f)((self.init_f)(), res);
            let mut input = input;
            while let Some((res, next)) = self.parser.parse(input) {
                state = (self.fold_f)(state, res);
                input = next;
            }

            Some((state, input))
        } else {
            None
        }
    }
}

pub struct RepeatFoldMut<TI, TO, P, FI, FF> {
    parser: P,
    init_f: FI,
    fold_f: FF,
    spooky_ghost: PhantomData<(TI, TO)>,
}

impl<TI, TO, P, FI, FF> RepeatFoldMut<TI, TO, P, FI, FF> {
    pub fn new(parser: P, init_f: FI, fold_f: FF) -> Self {
        Self {
            parser,
            init_f,
            fold_f,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, TI, TO, P, FI, FF> Parser<'i, TO> for RepeatFoldMut<TI, TO, P, FI, FF>
where
    P: Parser<'i, TI>,
    FI: Fn() -> TO,
    FF: Fn(&mut TO, TI),
{
    fn parse(&self, input: &'i [u8]) -> Option<(TO, &'i [u8])> {
        if let Some((res, input)) = self.parser.parse_first(input) {
            let mut state = (self.init_f)();
            (self.fold_f)(&mut state, res);

            let mut input = input;
            while let Some((res, next)) = self.parser.parse(input) {
                (self.fold_f)(&mut state, res);
                input = next;
            }

            Some((state, input))
        } else {
            None
        }
    }
}
