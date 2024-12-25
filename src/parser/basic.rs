use super::Parser;

pub fn everything<'i>() -> impl Parser<'i, &'i [u8]> {
    Everything
}

pub fn word<'i>() -> impl Parser<'i, &'i [u8]> {
    EverythingUntilChar(b' ', false)
}

pub fn line<'i>() -> impl Parser<'i, &'i [u8]> {
    EverythingUntilChar(b'\n', true)
}

pub fn word_terminated_by<'i>(ch: u8) -> impl Parser<'i, &'i [u8]> {
    EverythingUntilChar(ch, false)
}

pub fn n_bytes<'i, const N: usize>() -> impl Parser<'i, [u8; N]> {
    ByteArray::<N>
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

struct EverythingUntilChar(u8, bool);

impl<'i> Parser<'i, &'i [u8]> for EverythingUntilChar {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(&'i [u8], &'i [u8])> {
        if input.is_empty() {
            return None;
        }

        match input.iter().position(|&c| c == self.0) {
            Some(i) => {
                if i > 0 || !self.1 {
                    Some((&input[..i], &input[if self.1 { i + 1 } else { i }..]))
                } else {
                    None
                }
            }
            _ => Some((
                input,
                &input[if self.1 { input.len() + 1 } else { input.len() }..],
            )),
        }
    }
}

struct ByteArray<const N: usize>;

impl<'i, const N: usize> Parser<'i, [u8; N]> for ByteArray<N> {
    fn parse(&self, input: &'i [u8]) -> Option<([u8; N], &'i [u8])> {
        if input.len() >= N {
            let mut res = [0u8; N];
            res.copy_from_slice(&input[..N]);
            Some((res, &input[N..]))
        } else {
            None
        }
    }
}
