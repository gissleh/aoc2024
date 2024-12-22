use super::{Cost, Key};
use crate::grid::{Grid, GridCoordinate};
use bit_vec::BitVec;
use num::Zero;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::hash_map::Entry;
use std::hash::Hash;

pub trait SeenSpace<S> {
    /// Reset and clear any state from the seen space. It should be
    /// indistinguishable from running with an already empty version
    /// of itself.
    fn reset(&mut self);
    /// Returns the value of the seen state.
    fn has_seen(&self, state: &S) -> bool;
    /// Try to mark something as seen, returning true only if it passes
    /// it.
    fn try_mark_seen(&mut self, state: S) -> bool;
}

macro_rules! uint_bitset_space {
    ($typename: tt) => {
        impl<S> SeenSpace<S> for $typename
        where
            S: Key<usize>,
        {
            fn reset(&mut self) {
                *self = 0;
            }
            fn has_seen(&self, state: &S) -> bool {
                *self & 1 << state.key() != 0
            }
            fn try_mark_seen(&mut self, state: S) -> bool {
                let mask = 1 << state.key();
                if *self & mask == 0 {
                    *self |= mask;
                    true
                } else {
                    false
                }
            }
        }
    };
}

uint_bitset_space!(u8);
uint_bitset_space!(u16);
uint_bitset_space!(u32);
uint_bitset_space!(u64);
uint_bitset_space!(u128);

impl<S> SeenSpace<S> for BitVec
where
    S: Key<usize>,
{
    #[inline]
    fn reset(&mut self) {
        self.clear()
    }

    #[inline]
    fn has_seen(&self, state: &S) -> bool {
        self.get(state.key()) == Some(true)
    }

    #[inline]
    fn try_mark_seen(&mut self, state: S) -> bool {
        let index = state.key();
        if self.len() > index {
            self.grow(self.len() - index, false);
            self.push(true);
            true
        } else if self.get(index) == Some(false) {
            self.set(index, true);
            true
        } else {
            false
        }
    }
}

impl<S, C> SeenSpace<S> for Vec<C>
where
    S: Key<usize> + Cost<C>,
    C: Zero + Copy + Eq + Ord,
{
    fn reset(&mut self) {
        self.fill(C::zero());
    }

    fn has_seen(&self, state: &S) -> bool {
        if let Some(existing_cost) = self.get(state.key()) {
            !existing_cost.is_zero() && state.cost() >= *existing_cost
        } else {
            panic!("seen space vec out of range")
        }
    }

    fn try_mark_seen(&mut self, state: S) -> bool {
        if let Some(existing_cost) = self.get_mut(state.key()) {
            let state_cost = state.cost();
            if existing_cost.is_zero() || state_cost < *existing_cost {
                *existing_cost = state_cost;
                true
            } else {
                false
            }
        } else {
            panic!("seen space vec out of range")
        }
    }
}

impl<S, C, const N: usize> SeenSpace<S> for [C; N]
where
    S: Key<usize> + Cost<C>,
    C: Zero + Copy + Eq + Ord,
{
    fn reset(&mut self) {
        self.fill(C::zero());
    }

    fn has_seen(&self, state: &S) -> bool {
        if let Some(existing_cost) = self.get(state.key()) {
            !existing_cost.is_zero() && state.cost() >= *existing_cost
        } else {
            panic!("seen space vec out of range")
        }
    }

    fn try_mark_seen(&mut self, state: S) -> bool {
        if let Some(existing_cost) = self.get_mut(state.key()) {
            let state_cost = state.cost();
            if existing_cost.is_zero() || state_cost < *existing_cost {
                *existing_cost = state_cost;
                true
            } else {
                false
            }
        } else {
            panic!("seen space vec out of range")
        }
    }
}

impl<S, K> SeenSpace<S> for FxHashSet<K>
where
    S: Key<K>,
    K: Hash + Eq,
{
    #[inline]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline]
    fn has_seen(&self, state: &S) -> bool {
        self.contains(&state.key())
    }

    #[inline]
    fn try_mark_seen(&mut self, state: S) -> bool {
        self.insert(state.key())
    }
}

impl<S, K, C> SeenSpace<S> for FxHashMap<K, C>
where
    S: Key<K> + Cost<C>,
    K: Hash + Eq,
    C: Ord,
{
    #[inline]
    fn reset(&mut self) {
        self.clear();
    }

    #[inline]
    fn has_seen(&self, state: &S) -> bool {
        match self.get(&state.key()) {
            Some(existing_cost) => state.cost() >= *existing_cost,
            None => false,
        }
    }

    #[inline]
    fn try_mark_seen(&mut self, state: S) -> bool {
        match self.entry(state.key()) {
            Entry::Occupied(mut e) => {
                let state_cost = state.cost();
                let existing_cost = e.get_mut();
                if *existing_cost > state_cost {
                    *existing_cost = state_cost;
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(e) => {
                e.insert(state.cost());
                true
            }
        }
    }
}

pub struct ReEntrantSeenMap<K, C> {
    hash_map: FxHashMap<K, C>,
}

impl<K, C> ReEntrantSeenMap<K, C> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            hash_map: FxHashMap::with_capacity_and_hasher(cap, Default::default()),
        }
    }
}

impl<S, K, C> SeenSpace<S> for ReEntrantSeenMap<K, C>
where
    S: Key<K> + Cost<C>,
    K: Hash + Eq,
    C: Ord,
{
    #[inline]
    fn reset(&mut self) {
        self.hash_map.clear();
    }

    #[inline]
    fn has_seen(&self, state: &S) -> bool {
        match self.hash_map.get(&state.key()) {
            Some(existing_cost) => state.cost() > *existing_cost,
            None => false,
        }
    }

    #[inline]
    fn try_mark_seen(&mut self, state: S) -> bool {
        match self.hash_map.entry(state.key()) {
            Entry::Occupied(mut e) => {
                let state_cost = state.cost();
                let existing_cost = e.get_mut();
                if *existing_cost >= state_cost {
                    *existing_cost = state_cost;
                    true
                } else {
                    false
                }
            }
            Entry::Vacant(e) => {
                e.insert(state.cost());
                true
            }
        }
    }
}

impl<C, S, T> SeenSpace<S> for Grid<C, S, T>
where
    C: GridCoordinate,
    S: Key<C> + Cost<T> + AsRef<[T]> + AsMut<[T]>,
    T: Zero + Copy + Ord,
{
    fn reset(&mut self) {
        self.clear();
    }

    fn has_seen(&self, state: &S) -> bool {
        if let Some(existing_cost) = self.cell(&state.key()) {
            existing_cost.is_zero()
        } else {
            true
        }
    }

    fn try_mark_seen(&mut self, state: S) -> bool {
        if let Some(existing_cost) = self.cell_mut(&state.key()) {
            let state_cost = state.cost();
            if existing_cost.is_zero() || state_cost < *existing_cost {
                *existing_cost = state_cost;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

pub struct NoSeenSpace;

impl<S> SeenSpace<S> for NoSeenSpace {
    fn reset(&mut self) {}

    fn has_seen(&self, _: &S) -> bool {
        false
    }

    fn try_mark_seen(&mut self, _: S) -> bool {
        true
    }
}

pub struct BitArrSeenSpace<C, const N: usize> {
    data: [u32; N],
    size: C,
}

impl<const N: usize, C> BitArrSeenSpace<C, N>
where
    C: GridCoordinate,
{
    pub fn new(size: C) -> Self {
        #[cfg(debug_assertions)]
        assert!(
            size.area() < N * 32,
            "BitArrSeenSpace too small (size{}, cap={})",
            size.area(),
            N * 32
        );

        Self { data: [0; N], size }
    }
}

impl<const N: usize, C> BitArrSeenSpace<C, N>
where
    C: GridCoordinate,
{
    #[inline]
    fn index_of(&self, pos: C) -> (usize, u32) {
        let index = pos.index(&self.size);
        (index / 32, (index % 32) as u32)
    }
}

impl<const N: usize, S, C> SeenSpace<S> for BitArrSeenSpace<C, N>
where
    S: Key<C>,
    C: GridCoordinate,
{
    fn reset(&mut self) {
        self.data.fill(0);
    }

    fn has_seen(&self, state: &S) -> bool {
        let (index, bit) = self.index_of(state.key());
        self.data[index] & (1 << bit) != 0
    }

    fn try_mark_seen(&mut self, state: S) -> bool {
        let (index, bit) = self.index_of(state.key());

        if self.data[index] & (1 << bit) == 0 {
            self.data[index] |= 1 << bit;
            true
        } else {
            false
        }
    }
}
