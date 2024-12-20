use num::traits::{WrappingAdd, WrappingSub};
use num::One;
use std::cmp::minmax;
use std::ops::{Add, Sub};

#[allow(dead_code)]
pub trait CardinalNeighbors<T>: Copy {
    fn cardinal_neighbors(&self) -> [Self; 4];
    fn cardinal_neighbors_n(&self, n: T) -> [Self; 4];
}

impl<T> CardinalNeighbors<T> for (T, T)
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    fn cardinal_neighbors(&self) -> [Self; 4] {
        self.cardinal_neighbors_n(T::one())
    }

    fn cardinal_neighbors_n(&self, n: T) -> [Self; 4] {
        let (x, y) = *self;
        [(x, y - n), (x - n, y), (x + n, y), (x, y + n)]
    }
}

impl<T> CardinalNeighbors<T> for [T; 2]
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    fn cardinal_neighbors(&self) -> [Self; 4] {
        self.cardinal_neighbors_n(T::one())
    }

    fn cardinal_neighbors_n(&self, n: T) -> [Self; 4] {
        let [x, y] = *self;
        [[x, y - n], [x - n, y], [x + n, y], [x, y + n]]
    }
}

#[allow(dead_code)]
pub trait CardinalNeighborsWrapping<T>: Copy {
    fn cardinal_neighbors_wrapping(&self) -> [Self; 4];
    fn cardinal_neighbors_wrapping_n(&self, n: T) -> [Self; 4];
}

impl<T> CardinalNeighborsWrapping<T> for (T, T)
where
    T: Copy + WrappingAdd<Output = T> + WrappingSub<Output = T> + One,
{
    fn cardinal_neighbors_wrapping(&self) -> [Self; 4] {
        self.cardinal_neighbors_wrapping_n(T::one())
    }

    fn cardinal_neighbors_wrapping_n(&self, n: T) -> [Self; 4] {
        let (x, y) = *self;
        [
            (x, y.wrapping_sub(&n)),
            (x.wrapping_sub(&n), y),
            (x.wrapping_add(&n), y),
            (x, y.wrapping_add(&n)),
        ]
    }
}

impl<T> CardinalNeighborsWrapping<T> for [T; 2]
where
    T: Copy + WrappingAdd<Output = T> + WrappingSub<Output = T> + One,
{
    fn cardinal_neighbors_wrapping(&self) -> [Self; 4] {
        self.cardinal_neighbors_wrapping_n(T::one())
    }

    fn cardinal_neighbors_wrapping_n(&self, n: T) -> [Self; 4] {
        let [x, y] = *self;
        [
            [x, y.wrapping_sub(&n)],
            [x.wrapping_sub(&n), y],
            [x.wrapping_add(&n), y],
            [x, y.wrapping_add(&n)],
        ]
    }
}

#[allow(dead_code)]
pub trait XNeighbors: Copy {
    fn x_neighbors(&self) -> [Self; 4];
}

impl<T> XNeighbors for (T, T)
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    fn x_neighbors(&self) -> [Self; 4] {
        let one = T::one();
        let (x, y) = *self;

        [
            (x - one, y - one),
            (x + one, y - one),
            (x - one, y + one),
            (x + one, y + one),
        ]
    }
}

impl<T> XNeighbors for [T; 2]
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    fn x_neighbors(&self) -> [Self; 4] {
        let one = T::one();
        let [x, y] = *self;

        [
            [x - one, y - one],
            [x + one, y - one],
            [x - one, y + one],
            [x + one, y + one],
        ]
    }
}

#[allow(dead_code)]
pub trait Neighbors2D: Copy {
    fn neighbors_2d(&self) -> [Self; 8];
}

impl<T> Neighbors2D for (T, T)
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    fn neighbors_2d(&self) -> [Self; 8] {
        let one = T::one();
        let (x, y) = *self;

        [
            (x - one, y - one),
            (x, y - one),
            (x + one, y - one),
            (x - one, y),
            (x + one, y),
            (x - one, y + one),
            (x, y + one),
            (x + one, y + one),
        ]
    }
}

impl<T> Neighbors2D for [T; 2]
where
    T: Copy + Add<Output = T> + Sub<Output = T> + One,
{
    fn neighbors_2d(&self) -> [Self; 8] {
        let one = T::one();
        let [x, y] = *self;

        [
            [x - one, y - one],
            [x, y - one],
            [x + one, y - one],
            [x - one, y],
            [x + one, y],
            [x - one, y + one],
            [x, y + one],
            [x + one, y + one],
        ]
    }
}

pub trait ManhattanDistance: Copy {
    type Component;

    fn manhattan_distance_to(&self, other: &Self) -> Self::Component;
}

impl<C> ManhattanDistance for (C, C)
where
    C: Copy + Ord + Add<Output = C> + Sub<Output = C>,
{
    type Component = C;

    fn manhattan_distance_to(&self, other: &Self) -> Self::Component {
        let [x1, x2] = minmax(self.0, other.0);
        let [y1, y2] = minmax(self.1, other.1);
        (x2 - x1) + (y2 - y1)
    }
}

impl<C> ManhattanDistance for (C, C, C)
where
    C: Copy + Ord + Add<Output = C> + Sub<Output = C>,
{
    type Component = C;

    fn manhattan_distance_to(&self, other: &Self) -> Self::Component {
        let [x1, x2] = minmax(self.0, other.0);
        let [y1, y2] = minmax(self.1, other.1);
        let [z1, z2] = minmax(self.2, other.2);
        (x2 - x1) + (y2 - y1) + (z2 - z1)
    }
}

pub fn manhattan_distance<C, P: ManhattanDistance<Component = C>>(p1: &P, p2: &P) -> C {
    p1.manhattan_distance_to(p2)
}
