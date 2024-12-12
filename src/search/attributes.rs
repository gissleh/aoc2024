use arrayvec::ArrayVec;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::simd::{LaneCount, Simd, SimdElement, SupportedLaneCount};

pub trait Key<K> {
    fn key(&self) -> K;
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct OnlyKey<K>(pub K);

impl<K> Key<K> for OnlyKey<K>
where
    K: Copy,
{
    fn key(&self) -> K {
        self.0
    }
}

pub trait Cost<C>
where
    C: Ord + Eq,
{
    fn cost(&self) -> C;
}

pub trait Heuristic<C>
where
    C: Ord + Eq,
{
    fn heuristic(&self) -> C;
}

impl<K, C> Key<K> for (K, C)
where
    K: Eq + Ord + Copy,
{
    fn key(&self) -> K {
        self.0
    }
}

impl<K, C> Cost<C> for (K, C)
where
    C: Eq + Ord + Copy,
{
    fn cost(&self) -> C {
        self.1
    }
}

impl<K, C> Cost<C> for (K, (C, C))
where
    C: Eq + Ord + Copy + Add<C, Output = C>,
{
    fn cost(&self) -> C {
        self.1 .0
    }
}

impl<K, C> Heuristic<C> for (K, (C, C))
where
    C: Eq + Ord + Copy + Add<C, Output = C>,
{
    fn heuristic(&self) -> C {
        self.1 .1
    }
}

impl<K, C, EXTRA> Key<K> for (K, C, EXTRA)
where
    K: Eq + Ord + Copy,
{
    fn key(&self) -> K {
        self.0
    }
}

impl<K, C, EXTRA> Cost<C> for (K, C, EXTRA)
where
    C: Eq + Ord + Copy,
{
    fn cost(&self) -> C {
        self.1
    }
}

impl<T, const N: usize> Key<[T; N]> for ArrayVec<T, N>
where
    T: Copy + Default,
{
    fn key(&self) -> [T; N] {
        let mut res = [T::default(); N];
        res.copy_from_slice(self.as_slice());
        res
    }
}

impl<T, const N: usize> Key<[T; N]> for [T; N]
where
    T: Copy,
{
    fn key(&self) -> [T; N] {
        *self
    }
}

impl<T, const N: usize> Key<[T; N]> for Simd<T, N>
where
    T: SimdElement + Copy,
    LaneCount<N>: SupportedLaneCount,
{
    fn key(&self) -> [T; N] {
        *self.as_array()
    }
}

macro_rules! self_key {
    ($typename: tt) => {
        impl Key<$typename> for $typename {
            fn key(&self) -> $typename {
                *self
            }
        }
    };
}

self_key!(u8);
self_key!(u16);
self_key!(u32);
self_key!((u32, u32));
self_key!((u32, u32, u32));
self_key!((u32, u32, u32, u32));
self_key!(u64);
self_key!(u128);
self_key!(usize);
self_key!((usize, usize));
self_key!((usize, usize, usize));
self_key!((usize, usize, usize, usize));
self_key!(i8);
self_key!(i16);
self_key!(i32);
self_key!(i64);
self_key!(i128);
self_key!(isize);
self_key!((isize, isize));
self_key!((isize, isize, isize));
self_key!((isize, isize, isize, isize));

#[derive(Copy, Clone)]
pub struct KE<K, E>(pub K, pub E);

impl<K, E> Key<K> for KE<K, E>
where
    K: Copy,
{
    fn key(&self) -> K {
        self.0
    }
}

impl<K, E> Hash for KE<K, E>
where
    K: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<K, E> PartialEq for KE<K, E>
where
    K: PartialEq,
{
    fn eq(&self, other: &KE<K, E>) -> bool {
        self.0 == other.0
    }
}

impl<K, E> Eq for KE<K, E> where K: Eq {}
