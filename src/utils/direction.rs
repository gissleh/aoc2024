use num::traits::WrappingSub;
use num::One;
use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
pub enum CardinalDirection {
    West,
    North,
    East,
    South,
}

impl CardinalDirection {
    /// Get a bit that's unique to the direction, in clockwise order from west to south.
    pub fn to_bits(&self) -> u8 {
        match *self {
            CardinalDirection::West => 0b0001,
            CardinalDirection::North => 0b0010,
            CardinalDirection::East => 0b0100,
            CardinalDirection::South => 0b1000,
        }
    }

    /// Get the next position in this direction
    pub fn next_pos<T>(&self, pos: &(T, T)) -> (T, T)
    where
        T: WrappingSub + Copy + Add<T, Output = T> + One,
    {
        let one = T::one();

        match self {
            CardinalDirection::West => (pos.0.wrapping_sub(&one), pos.1),
            CardinalDirection::North => (pos.0, pos.1.wrapping_sub(&one)),
            CardinalDirection::East => (pos.0 + one, pos.1),
            CardinalDirection::South => (pos.0, pos.1 + one),
        }
    }

    /// Turn against the clock
    pub fn turn_anticlockwise(&self) -> CardinalDirection {
        match self {
            CardinalDirection::West => CardinalDirection::South,
            CardinalDirection::North => CardinalDirection::West,
            CardinalDirection::East => CardinalDirection::North,
            CardinalDirection::South => CardinalDirection::East,
        }
    }

    /// Turn with the clock
    pub fn turn_clockwise(&self) -> CardinalDirection {
        match self {
            CardinalDirection::West => CardinalDirection::North,
            CardinalDirection::North => CardinalDirection::East,
            CardinalDirection::East => CardinalDirection::South,
            CardinalDirection::South => CardinalDirection::West,
        }
    }
}
