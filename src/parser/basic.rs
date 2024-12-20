use super::Parser;

pub fn everything<'i>() -> impl Parser<'i, &'i [u8]> {
    Everything
}

pub fn word<'i>() -> impl Parser<'i, &'i [u8]> {
    EverythingUntilChar(b' ')
}

pub fn line<'i>() -> impl Parser<'i, &'i [u8]> {
    EverythingUntilChar(b'\n')
}

pub fn word_terminated_by<'i>(ch: u8) -> impl Parser<'i, &'i [u8]> {
    EverythingUntilChar(ch)
}

struct Everything;

impl<'i> Parser<'i, &'i [u8]> for Everything {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(&'i [u8], &'i [u8])> {
        if !input.is_empty() {
            Some((input, &input[input.len()..]))
        } else {
            None
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        !input.is_empty()
    }

    #[inline]
    fn find_parsable(&self, input: &'i [u8]) -> Option<(&'i [u8], usize, &'i [u8])> {
        self.parse(input).map(|(res, next)| (res, 0, next))
    }
}

struct EverythingUntilChar(u8);

impl<'i> Parser<'i, &'i [u8]> for EverythingUntilChar {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(&'i [u8], &'i [u8])> {
        if input.is_empty() {
            return None;
        }

        match input.iter().position(|&c| c == self.0) {
            Some(i) => Some((&input[..i], &input[i + 1..])),
            None => Some((input, &input[input.len()..])),
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        !input.is_empty()
    }
}
