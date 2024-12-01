use super::Parser;
use std::marker::PhantomData;
use std::ops::{AddAssign, MulAssign, Neg};

#[inline]
pub fn hex_digit<'i>() -> impl Parser<'i, u8> + Copy + Clone {
    HexDigit
}

#[inline]
pub fn digit<'i>() -> impl Parser<'i, u8> + Copy + Clone {
    Base10Digit
}

#[inline]
pub fn unsigned_int<'i, T, DP>(radix: T, dp: DP) -> UnsignedInt<'i, T, DP>
where
    T: From<u8> + Copy + AddAssign<T> + MulAssign<T> + 'i,
    DP: Parser<'i, u8>,
{
    UnsignedInt(dp, radix, Default::default())
}

#[inline]
pub fn signed_int<'i, T, DP>(radix: T, dp: DP) -> SignedInt<'i, T, DP>
where
    T: From<u8> + Neg<Output = T> + Copy + AddAssign<T> + MulAssign<T> + 'i,
    DP: Parser<'i, u8>,
{
    SignedInt(dp, radix, Default::default())
}

#[inline]
pub fn uint<'i, T>() -> impl Parser<'i, T> + Clone + Copy
where
    T: From<u8> + Copy + AddAssign<T> + MulAssign<T> + 'i,
{
    UnsignedInt(digit(), T::from(10), Default::default())
}

#[inline]
pub fn int<'i, T>() -> impl Parser<'i, T> + Clone + Copy
where
    T: From<u8> + Neg<Output = T> + Copy + AddAssign<T> + MulAssign<T> + 'i,
{
    SignedInt(digit(), T::from(10), Default::default())
}

pub struct UnsignedInt<'i, T, DP>(DP, T, PhantomData<&'i T>);

impl<'i, T, DP> Clone for UnsignedInt<'i, T, DP>
where
    DP: Clone + 'i,
    T: Clone + 'i,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), Default::default())
    }
}

impl<'i, T, DP> Copy for UnsignedInt<'i, T, DP>
where
    DP: Copy + 'i,
    T: Copy + 'i,
{
}

impl<'i, T, DP> Parser<'i, T> for UnsignedInt<'i, T, DP>
where
    T: From<u8> + Copy + AddAssign<T> + MulAssign<T>,
    DP: Parser<'i, u8>,
{
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        let (digit, mut input) = self.0.parse(input)?;
        let mut number = T::from(digit);

        while let Some((digit, next)) = self.0.parse(input) {
            number.mul_assign(self.1);
            number += digit.into();
            input = next;
        }

        Some((number, input))
    }

    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        let mut input = self.0.parse_discard(input)?;

        while let Some(next) = self.0.parse_discard(input) {
            input = next;
        }

        Some(input)
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        self.0.can_parse(input)
    }
}

pub struct SignedInt<'i, T, DP>(DP, T, PhantomData<&'i T>);

impl<'i, T, DP> Clone for SignedInt<'i, T, DP>
where
    DP: Clone + 'i,
    T: Clone + 'i,
{
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone(), Default::default())
    }
}

impl<'i, T, DP> Copy for SignedInt<'i, T, DP>
where
    DP: Copy + 'i,
    T: Copy + 'i,
{
}

impl<'i, T, DP> Parser<'i, T> for SignedInt<'i, T, DP>
where
    T: From<u8> + Neg<Output = T> + Copy + AddAssign<T> + MulAssign<T>,
    DP: Parser<'i, u8>,
{
    fn parse(&self, input: &'i [u8]) -> Option<(T, &'i [u8])> {
        let (negate, input) = if input.get(0) == Some(&b'-') {
            (true, &input[1..])
        } else {
            (false, input)
        };

        let (digit, mut input) = self.0.parse(input)?;
        let mut number = T::from(digit);

        while let Some((digit, next)) = self.0.parse(input) {
            number.mul_assign(self.1);
            number += digit.into();
            input = next;
        }

        if negate {
            number = -number
        }

        Some((number, input))
    }

    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        let input = if input.get(0) == Some(&b'-') {
            &input[1..]
        } else {
            input
        };

        let mut input = self.0.parse_discard(input)?;

        while let Some(next) = self.0.parse_discard(input) {
            input = next;
        }

        Some(input)
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        input.get(0) == Some(&b'-') || self.0.can_parse(input)
    }
}

#[derive(Copy, Clone)]
struct HexDigit;

impl<'i> Parser<'i, u8> for HexDigit {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(u8, &'i [u8])> {
        let d = *input.first()?;
        match d {
            b'0'..=b'9' => Some((d - b'0', &input[1..])),
            b'a'..=b'f' => Some((d - b'a' + 10, &input[1..])),
            b'A'..=b'F' => Some((d - b'A' + 10, &input[1..])),
            _ => None,
        }
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        if self.can_parse(input) {
            Some(&input[1..])
        } else {
            None
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        if let Some(d) = input.first() {
            match d {
                b'0'..=b'9' => true,
                b'a'..=b'f' => true,
                b'A'..=b'F' => true,
                _ => false,
            }
        } else {
            false
        }
    }
}

#[derive(Copy, Clone)]
struct Base10Digit;

impl<'i> Parser<'i, u8> for Base10Digit {
    #[inline]
    fn parse(&self, input: &'i [u8]) -> Option<(u8, &'i [u8])> {
        let d = *input.first()?;
        match d {
            b'0'..=b'9' => Some((d - b'0', &input[1..])),
            _ => None,
        }
    }

    #[inline]
    fn parse_discard(&self, input: &'i [u8]) -> Option<&'i [u8]> {
        if self.can_parse(input) {
            Some(&input[1..])
        } else {
            None
        }
    }

    #[inline]
    fn can_parse(&self, input: &'i [u8]) -> bool {
        !input.is_empty() && (b'0'..=b'9').contains(&input[0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_digit() {
        assert_eq!(hex_digit().parse(b"dead"), Some((0xd, &b"ead"[..])));
        assert_eq!(hex_digit().parse(b"BEAD"), Some((0xb, &b"EAD"[..])));
        assert_eq!(hex_digit().parse(b"978"), Some((9, &b"78"[..])));
        assert_eq!(hex_digit().parse(b"grub"), None);
    }

    #[test]
    fn unsigned_int_usable() {
        assert_eq!(
            unsigned_int::<u16, _>(16, hex_digit()).parse(b"dead stuff"),
            Some((0xdead, &b" stuff"[..]))
        );
        assert_eq!(
            unsigned_int::<u32, _>(16, hex_digit()).parse(b"deadbeef"),
            Some((0xdeadbeef, &b""[..]))
        );
        assert_eq!(
            unsigned_int::<i32, _>(10, digit()).parse(b"1234"),
            Some((1234, &b""[..]))
        );
        assert_eq!(
            unsigned_int::<u64, _>(16, hex_digit()).parse(b"deadbeef"),
            Some((0xdeadbeef, &b""[..]))
        );
        assert_eq!(
            unsigned_int::<i32, _>(256, unsigned_int::<u8, _>(10, digit()).and_discard(b';'))
                .parse(b"1;2;3;4;"),
            Some((16909060, &b""[..]))
        );
    }

    #[test]
    fn signed_int_usable() {
        assert_eq!(
            signed_int::<i16, _>(16, hex_digit()).parse(b"666 + 123"),
            Some((0x666, &b" + 123"[..]))
        );
        assert_eq!(
            signed_int::<i32, _>(10, digit()).parse(b"1234"),
            Some((1234, &b""[..]))
        );
        assert_eq!(
            signed_int::<i32, _>(10, digit()).parse(b"-1234"),
            Some((-1234, &b""[..]))
        );
        assert_eq!(
            signed_int::<i64, _>(16, hex_digit()).parse(b"-ea7beef"),
            Some((-0xea7beef, &b""[..]))
        );
    }

    #[test]
    fn unsigned_int_composed() {
        let number = uint::<u8>();
        let ip = number
            .and_discard(b'.')
            .and(number)
            .and_discard(b'.')
            .and(number)
            .and_discard(b'.')
            .and(number);

        let _can_copy = ip;

        assert_eq!(
            ip.parse(b"224.128.64.197:1234"),
            Some(((((224, 128), 64), 197), &b":1234"[..]))
        );
    }
}
