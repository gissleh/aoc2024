use std::ops::{Index, IndexMut};

pub struct Grid<C, S, T>
where
    C: GridCoordinate,
{
    size: C,
    storage: S,
    default: T,
}

impl<C, S, T> Grid<C, S, T>
where
    C: GridCoordinate,
    S: AsMut<[T]>,
    T: Copy,
{
    #[inline]
    pub fn clear(&mut self) {
        self.storage.as_mut().fill(self.default)
    }
}

impl<C, S, T> Grid<C, S, T>
where
    C: GridCoordinate,
    S: AsRef<[T]>,
{
    #[inline]
    pub fn size(&self) -> &C {
        &self.size
    }

    #[inline]
    pub fn cell(&self, pos: &C) -> Option<&T> {
        if pos.in_bounds(&self.size) {
            self.storage.as_ref().get(pos.index(&self.size))
        } else {
            None
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = (C, &T)> {
        GridIter {
            data: self.storage.as_ref(),
            len: self.size.area(),
            coord: C::zero(),
            size: self.size,
            index: 0,
        }
    }
}

impl<C, S, T> Index<C> for Grid<C, S, T>
where
    C: GridCoordinate,
    S: Index<usize, Output = T>,
    T: Copy,
{
    type Output = T;

    fn index(&self, index: C) -> &Self::Output {
        &self.storage[index.index(&self.size)]
    }
}

impl<C, S, T> Grid<C, S, T>
where
    C: GridCoordinate,
    S: AsMut<[T]>,
{
    #[inline]
    pub fn cell_mut(&mut self, pos: &C) -> Option<&mut T> {
        if pos.in_bounds(&self.size) {
            self.storage.as_mut().get_mut(pos.index(&self.size))
        } else {
            None
        }
    }
}

impl<C, S, T> IndexMut<C> for Grid<C, S, T>
where
    C: GridCoordinate,
    S: IndexMut<usize, Output = T>,
    T: Copy,
{
    fn index_mut(&mut self, index: C) -> &mut Self::Output {
        &mut self.storage[index.index(&self.size)]
    }
}

impl<C, S, T> Grid<C, S, T>
where
    C: GridCoordinate,
    S: AsRef<[T]>,
{
    pub fn as_slice(&self) -> &[T] {
        self.storage.as_ref()
    }

    pub fn new_with_default(size: C, storage: S, default: T) -> Self {
        assert!(
            storage.as_ref().len() >= size.area(),
            "storage len: {} < size expected: {}",
            storage.as_ref().len(),
            size.area()
        );
        Self {
            size,
            storage,
            default,
        }
    }
}

impl<C, S, T> Grid<C, S, T>
where
    C: GridCoordinate,
    T: Default + Clone,
    S: AsRef<[T]>,
{
    pub fn with_storage(size: C, storage: S) -> Self {
        Self::new_with_default(size, storage, T::default())
    }
}

impl<C, T> Grid<C, Vec<T>, T>
where
    C: GridCoordinate,
    T: Default + Clone,
{
    pub fn new_vec(size: C) -> Self {
        Self::new_with_default(size, vec![T::default(); size.area()], T::default())
    }
}

struct GridIter<'g, C, T> {
    coord: C,
    size: C,
    index: usize,
    len: usize,
    data: &'g [T],
}

impl<'g, C, T> Iterator for GridIter<'g, C, T>
where
    C: GridCoordinate,
{
    type Item = (C, &'g T);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let curr = &self.data[self.index];
            let coord = self.coord;

            self.coord = self.coord.next(&self.size);
            self.index += 1;

            Some((coord, curr))
        } else {
            None
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len - self.index, Some(self.len - self.index))
    }
}

pub trait GridCoordinate: Default + Copy {
    fn zero() -> Self;
    fn area(&self) -> usize;
    fn in_bounds(&self, size: &Self) -> bool;
    fn index(&self, size: &Self) -> usize;
    fn next(&self, size: &Self) -> Self;
}

macro_rules! impl_coord {
    ($int: tt) => {
        impl GridCoordinate for $int {
            #[inline]
            fn zero() -> Self {
                0
            }

            #[inline]
            fn area(&self) -> usize {
                *self as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                *self < *size
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                (*self % *size) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                (*self + 1) % *size
            }
        }

        impl GridCoordinate for ($int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 < size.0 && self.1 < size.1
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                ((self.1 * size.0) + self.0) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    (0, self.1 + 1)
                } else {
                    (nx, self.1)
                }
            }
        }

        impl GridCoordinate for ($int, $int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1 * self.2) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 < size.0 && self.1 < size.1 && self.2 < size.2
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                ((self.2 * (size.0 * size.1)) + (self.1 * size.0) + self.0) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    let ny = self.1 + 1;
                    if ny == size.1 {
                        (0, 0, self.2 + 1)
                    } else {
                        (0, ny, self.2)
                    }
                } else {
                    (nx, self.1, self.2)
                }
            }
        }

        impl GridCoordinate for ($int, $int, $int, $int) {
            #[inline]
            fn zero() -> Self {
                (0, 0, 0, 0)
            }

            #[inline]
            fn area(&self) -> usize {
                (self.0 * self.1 * self.2 * self.3) as usize
            }

            #[inline]
            fn in_bounds(&self, size: &Self) -> bool {
                self.0 < size.0 && self.1 < size.1 && self.2 < size.2 && self.3 < size.3
            }

            #[inline]
            fn index(&self, size: &Self) -> usize {
                let w = size.0;
                let wh = w * size.1;
                let whd = wh * size.2;

                ((self.3 * whd) + (self.2 * wh) + (self.1 * size.0) + self.0) as usize
            }

            #[inline]
            fn next(&self, size: &Self) -> Self {
                let nx = self.0 + 1;
                if nx == size.0 {
                    let ny = self.1 + 1;
                    if ny == size.1 {
                        let nz = self.2 + 1;
                        if nz == size.2 {
                            (0, 0, 0, self.3 + 1)
                        } else {
                            (0, 0, nz, self.3)
                        }
                    } else {
                        (0, ny, self.2, self.3)
                    }
                } else {
                    (nx, self.1, self.2, self.3)
                }
            }
        }
    };
}

impl_coord!(u8);
impl_coord!(u16);
impl_coord!(u32);
impl_coord!(u64);
impl_coord!(u128);
impl_coord!(usize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_cells() {
        let mut grid = Grid::with_storage((4u8, 4u8), vec![0; 16]);
        *grid.cell_mut(&(2, 2)).unwrap() = 64;
        *grid.cell_mut(&(3, 3)).unwrap() = 96;
        *grid.cell_mut(&(1, 2)).unwrap() = 24;
        *grid.cell_mut(&(2, 1)).unwrap() = 55;
        *grid.cell_mut(&(3, 0)).unwrap() = 96;

        assert_eq!(*grid.cell(&(3, 0)).unwrap(), 96);
        assert_eq!(*grid.cell(&(0, 0)).unwrap(), 0);

        assert_eq!(
            grid.storage.as_slice(),
            &[0, 0, 0, 96, 0, 0, 55, 0, 0, 24, 64, 0, 0, 0, 0, 96]
        );
    }

    #[test]
    fn coords_increment_correctly() {
        let mut coord = 0u32;
        let size = 100000u32;
        assert_eq!(size.area(), 100000);
        for index in 0..size.area() {
            assert_eq!(coord.index(&size), index);
            coord = coord.next(&size);
        }

        let mut coord = (0u32, 0u32);
        let size = (1024, 1024);
        assert_eq!(size.area(), 1048576);
        for index in 0..size.area() {
            assert_eq!(coord.index(&size), index);
            coord = coord.next(&size);
        }

        let mut coord = (0u32, 0u32, 0u32);
        let size = (100, 100, 100);
        assert_eq!(size.area(), 1000000);
        for index in 0..size.area() {
            assert_eq!(coord.index(&size), index);
            coord = coord.next(&size);
        }

        let mut coord = (0u32, 0u32, 0u32, 0u32);
        let size = (54, 33, 66, 12);
        assert_eq!(size.area(), 1411344);
        for index in 0..size.area() {
            assert_eq!(coord.index(&size), index);
            coord = coord.next(&size);
        }
    }

    #[test]
    fn grid_iter_works() {
        let grid_2d = Grid::with_storage(
            (4usize, 4usize),
            [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        );
        assert_eq!(
            grid_2d.iter().collect::<Vec<_>>(),
            vec![
                ((0, 0), &0),
                ((1, 0), &1),
                ((2, 0), &2),
                ((3, 0), &3),
                ((0, 1), &4),
                ((1, 1), &5),
                ((2, 1), &6),
                ((3, 1), &7),
                ((0, 2), &8),
                ((1, 2), &9),
                ((2, 2), &10),
                ((3, 2), &11),
                ((0, 3), &12),
                ((1, 3), &13),
                ((2, 3), &14),
                ((3, 3), &15),
            ]
        );

        let grid_3d = Grid::with_storage((2u8, 2u8, 2u8), [0u8, 1, 2, 3, 4, 5, 6, 7].as_slice());
        assert_eq!(
            grid_3d.iter().collect::<Vec<_>>(),
            vec![
                ((0, 0, 0), &0u8),
                ((1, 0, 0), &1u8),
                ((0, 1, 0), &2u8),
                ((1, 1, 0), &3u8),
                ((0, 0, 1), &4u8),
                ((1, 0, 1), &5u8),
                ((0, 1, 1), &6u8),
                ((1, 1, 1), &7u8),
            ]
        );
    }
}
