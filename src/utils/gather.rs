use arrayvec::ArrayVec;
use rustc_hash::FxHashMap;
use std::hash::Hash;

pub trait GatherTarget<T> {
    fn init_gather_target(size_hint: usize) -> Self;
    /// Gather will always push to previous index + 1.
    fn gather(&mut self, i: usize, value: T) -> bool;
}

impl<T> GatherTarget<T> for Vec<T> {
    #[inline]
    fn init_gather_target(size_hint: usize) -> Self {
        let size_hint = if size_hint == 0 { 16 } else { size_hint };
        Vec::with_capacity(size_hint)
    }

    #[inline]
    fn gather(&mut self, _: usize, value: T) -> bool {
        self.push(value);
        true
    }
}

impl<T, const N: usize> GatherTarget<T> for ArrayVec<T, N> {
    #[inline]
    fn init_gather_target(_: usize) -> Self {
        ArrayVec::new()
    }

    #[inline]
    fn gather(&mut self, _: usize, value: T) -> bool {
        self.push(value);
        !self.is_full()
    }
}

impl<T, const N: usize> GatherTarget<T> for [T; N]
where
    T: Copy + Default,
{
    #[inline]
    fn init_gather_target(_: usize) -> Self {
        [Default::default(); N]
    }

    #[inline]
    fn gather(&mut self, i: usize, value: T) -> bool {
        self[i] = value;
        i < N - 1
    }
}

impl<T> GatherTarget<T> for () {
    #[inline]
    fn init_gather_target(_size_hint: usize) -> Self {
        ()
    }
    #[inline]
    fn gather(&mut self, _i: usize, _value: T) -> bool {
        false
    }
}

impl<T> GatherTarget<T> for (T, T)
where
    T: Copy + Default,
{
    #[inline]
    fn init_gather_target(_: usize) -> Self {
        Default::default()
    }

    #[inline]
    fn gather(&mut self, i: usize, value: T) -> bool {
        match i {
            0 => {
                self.0 = value;
                true
            }
            1 => {
                self.1 = value;
                false
            }
            _ => false,
        }
    }
}

impl<T> GatherTarget<T> for (T, T, T)
where
    T: Copy + Default,
{
    #[inline]
    fn init_gather_target(_: usize) -> Self {
        Default::default()
    }

    #[inline]
    fn gather(&mut self, i: usize, value: T) -> bool {
        match i {
            0 => {
                self.0 = value;
                true
            }
            1 => {
                self.1 = value;
                true
            }
            2 => {
                self.2 = value;
                false
            }
            _ => false,
        }
    }
}

impl<T> GatherTarget<T> for (T, T, T, T)
where
    T: Copy + Default,
{
    #[inline]
    fn init_gather_target(_: usize) -> Self {
        Default::default()
    }

    #[inline]
    fn gather(&mut self, i: usize, value: T) -> bool {
        match i {
            0 => {
                self.0 = value;
                true
            }
            1 => {
                self.1 = value;
                true
            }
            2 => {
                self.2 = value;
                true
            }
            3 => {
                self.3 = value;
                false
            }
            _ => false,
        }
    }
}

impl<K, V> GatherTarget<(K, V)> for FxHashMap<K, V>
where
    K: Eq + Hash,
{
    #[inline]
    fn init_gather_target(size_hint: usize) -> Self {
        let size_hint = if size_hint == 0 { 16 } else { size_hint };
        FxHashMap::with_capacity_and_hasher(size_hint, Default::default())
    }

    #[inline]
    fn gather(&mut self, _: usize, value: (K, V)) -> bool {
        self.insert(value.0, value.1);
        true
    }
}

impl GatherTarget<()> for usize {
    #[inline]
    fn init_gather_target(_: usize) -> Self {
        0
    }

    #[inline]
    fn gather(&mut self, i: usize, _: ()) -> bool {
        *self = i + 1;
        true
    }
}
