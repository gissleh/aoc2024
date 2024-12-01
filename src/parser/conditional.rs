use crate::parser::Parser;
use std::marker::PhantomData;

pub struct OnlyIf<'i, P, T, F>(pub P, pub F, pub PhantomData<&'i T>)
where
    P: Parser<'i, T>,
    F: Fn(&T) -> bool;

impl<'i, P, T, F> Parser<'i, T> for OnlyIf<'i, P, T, F>
where
    P: Parser<'i, T>,
    F: Fn(&T) -> bool,
{
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        match self.0.parse(input) {
            Some((result, next)) if self.1(&result) => Some((result, next)),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::int;

    #[test]
    fn only_if_works() {
        assert_eq!(
            int().only_if(|v| *v > -10 && *v < 10).parse(b"1 stuff"),
            Some((1, b" stuff".as_ref())),
        );
        assert_eq!(
            int().only_if(|v| *v > -10 && *v < 10).parse(b"-7 stuffs"),
            Some((-7, b" stuffs".as_ref())),
        );
        assert_eq!(
            int::<i32>().parse(b"-17 stuffs"),
            Some((-17, b" stuffs".as_ref())),
        );
        assert_eq!(
            int::<i32>()
                .only_if(|v| *v > -10 && *v < 10)
                .parse(b"-17 stuffs"),
            None,
        );
    }
}
