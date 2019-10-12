/*
    Copyright 2010-2011 Ilkka Halila
    Copyright 2019 Alexander Krivács Schrøder

    This file is part of Goblin Camp.

    Goblin Camp is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Goblin Camp is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with Goblin Camp.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ops::{Index, Add, Sub, Mul, Div, AddAssign, SubAssign, IndexMut, MulAssign, DivAssign};
use std::fmt::Display;
use std::fmt;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
    NoDirection,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn both() -> impl Iterator<Item=Self> {
        use Axis::*;
        [X, Y].iter().cloned()
    }

    pub fn for_both<F>(f: F) where
        Self: Sized, F: FnMut(Self) {
        Self::both().for_each(f)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash, Default)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn min(self, other: Self) -> Self {
        Self { x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    pub fn max(self, other: Self) -> Self {
        Self { x: self.x.max(other.x), y: self.y.max(other.y) }
    }

    pub fn inside_rectangle(self, low: Self, high: Self) -> bool {
        self.x >= low.x && self.x <= high.x && self.y >= low.y && self.y <= high.y
    }

    pub fn inside_extent(self, origin: Self, extent: Self) -> bool {
        self.inside_rectangle(origin, origin + extent - 1)
    }

    pub fn on_rectangle_edges(self, low: Self, high: Self) -> bool {
        self.x == low.x || self.x == high.x || self.y == low.y || self.y == high.y
    }

    pub fn on_extent_edges(self, origin: Self, extent: Self) -> bool {
        self.on_rectangle_edges(origin, origin + extent - 1)
    }

    pub fn axes_mut(&mut self) -> [&mut i32; 2] {
        [&mut self.x, &mut self.y]
    }

    pub fn axes(self) -> [i32; 2] {
        [self.x, self.y]
    }

    // TODO: Rename to `clamp_to_rectangle` or similar. What is does is ensure we are inside the given bounds.
    pub fn shrink_rectangle(self, low: Self, high: Self) -> Self {
        let mut res = self;
        Axis::for_both(|a| res[a] = low[a].max(high[a].min(res[a])));

        res
    }

    pub fn shrink_extent(self, origin: Self, extent: Self) -> Self {
        self.shrink_rectangle(origin, origin + extent - 1)
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<Direction> for Coordinate {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => { Self::new(0, -1) }
            Direction::NorthEast => { Self::new(1, -1) }
            Direction::East => { Self::new(1, 0) }
            Direction::SouthEast => { Self::new(1, 1) }
            Direction::South => { Self::new(0, 1) }
            Direction::SouthWest => { Self::new(-1, 1) }
            Direction::West => { Self::new(-1, 0) }
            Direction::NorthWest => { Self::new(-1, -1) }
            Direction::NoDirection => { Self::new(0, 0) }
        }
    }
}

impl Index<Axis> for Coordinate {
    type Output = i32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => { &self.x }
            Axis::Y => { &self.y }
        }
    }
}

impl IndexMut<Axis> for Coordinate {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => { &mut self.x }
            Axis::Y => { &mut self.y }
        }
    }
}

impl Add<i32> for Coordinate {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign<i32> for Coordinate {
    fn add_assign(&mut self, rhs: i32) {
        Axis::both().for_each(|a| self[a] += rhs);
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        Axis::both().for_each(|a| self[a] += rhs[a]);
    }
}

impl Sub<i32> for Coordinate {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self { x: self.x - rhs, y: self.y - rhs }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign<i32> for Coordinate {
    fn sub_assign(&mut self, rhs: i32) {
        Axis::both().for_each(|a| self[a] -= rhs);
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        Axis::both().for_each(|a| self[a] -= rhs[a]);
    }
}

impl Mul<i32> for Coordinate {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self { x: self.x * rhs, y: self.y * rhs }
    }
}

impl MulAssign<i32> for Coordinate {
    fn mul_assign(&mut self, rhs: i32) {
        Axis::both().for_each(|a| self[a] *= rhs);
    }
}

impl Div<i32> for Coordinate {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self { x: self.x / rhs, y: self.y / rhs }
    }
}

impl DivAssign<i32> for Coordinate {
    fn div_assign(&mut self, rhs: i32) {
        Axis::both().for_each(|a| self[a] /= rhs);
    }
}
