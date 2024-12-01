use crate::parser::Parser;
use std::marker::PhantomData;

pub struct DelimitedBy<PE, PD, TE, TD> {
    elem_parser: PE,
    delim_parser: PD,
    spooky_ghost: PhantomData<(TE, TD)>,
}

impl<PE, PD, TE, TD> DelimitedBy<PE, PD, TE, TD> {
    pub fn new(elem_parser: PE, delim_parser: PD) -> Self {
        Self {
            elem_parser,
            delim_parser,
            spooky_ghost: Default::default(),
        }
    }
}

impl<'i, PE, PD, TE, TD> Parser<'i, TE> for DelimitedBy<PE, PD, TE, TD>
where
    PE: Parser<'i, TE>,
    PD: Parser<'i, TD>,
{
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(TE, &'i [u8])> {
        self.delim_parser
            .parse_discard(input)
            .and_then(|next| self.elem_parser.parse(next))
    }

    #[inline]
    fn parse_first(&self, input: &'i [u8]) -> Option<(TE, &'i [u8])> {
        self.elem_parser.parse_first(input)
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.delim_parser
            .parse_discard(input)
            .and_then(|next| self.elem_parser.parse_discard(next))
    }

    #[inline]
    fn parse_discard_first(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        self.elem_parser.parse_discard_first(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::parser;
    use crate::parser::delimiter::DelimitedBy;
    use crate::parser::Parser;

    #[test]
    fn test_delimits_correctly() {
        let parser = DelimitedBy::new(parser::int::<i32>(), b',');

        assert_eq!(parser.parse_first(b","), None);
        assert_eq!(
            parser.parse_first(b"15,24"),
            Some((15i32, b",24".as_slice()))
        );
        assert_eq!(
            parser.repeat::<[i32; 3]>().parse(b"1,2,3"),
            Some(([1, 2, 3], b"".as_slice()))
        );
    }
}
