/*
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp Revival.

    Goblin Camp Revival is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp Revival is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp Revival.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Array2D<T> {
    raw: Vec<T>,
    rows: usize,
    columns: usize,
}

impl<T: Default + Clone> Array2D<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        Self {
            raw: vec![T::default(); rows * columns],
            rows,
            columns,
        }
    }
}

impl<T: Clone> Array2D<T> {
    pub fn new_with<F>(rows: usize, columns: usize, mut func: F) -> Self
    where
        F: FnMut(usize, usize) -> T,
    {
        let raw: Vec<_> = (0..rows.checked_mul(columns).expect("rows * colums > usize"))
            .map(|count| func(count / columns, count % columns))
            .collect();
        Self { raw, rows, columns }
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];

    fn index(&self, row: usize) -> &Self::Output {
        assert!(row < self.rows);
        &self.raw.as_slice()[row * self.columns..][..self.columns]
    }
}

impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        assert!(row < self.rows);
        &mut self.raw.as_mut_slice()[row * self.columns..][..self.columns]
    }
}

impl<T> Index<(usize, usize)> for Array2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Array2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self[index.0][index.1]
    }
}

pub mod extras {
    use crate::coordinate::Coordinate;
    use crate::util::Array2D;

    pub trait Array2DCoordinateAccessor<T> {
        fn by_coordinate(&self, p: Coordinate) -> &T;
        fn by_coordinate_mut(&mut self, p: Coordinate) -> &mut T;
    }

    impl<T> Array2DCoordinateAccessor<T> for Array2D<T> {
        fn by_coordinate(&self, p: Coordinate) -> &T {
            &self[p.x as usize][p.y as usize]
        }

        fn by_coordinate_mut(&mut self, p: Coordinate) -> &mut T {
            &mut self[p.x as usize][p.y as usize]
        }
    }
}
