/*
    Copyright 2010-2011 Ilkka Halila
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

use crate::data::base::{Position, Size};
use crate::data::random::Selection;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

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
    None,
}

impl Direction {
    pub fn reverse(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::NorthEast => Self::SouthWest,
            Self::East => Self::West,
            Self::SouthEast => Self::NorthWest,
            Self::South => Self::North,
            Self::SouthWest => Self::NorthEast,
            Self::West => Self::East,
            Self::NorthWest => Self::SouthEast,
            Self::None => Self::None,
        }
    }
}

const DIRECTIONS: [Direction; 8] = [
    Direction::North,
    Direction::NorthEast,
    Direction::East,
    Direction::SouthEast,
    Direction::South,
    Direction::SouthWest,
    Direction::West,
    Direction::NorthWest,
];

impl Selection for Direction {
    fn get_choices() -> &'static [Self] {
        &DIRECTIONS
    }
}

impl Default for Direction {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Axis {
    X,
    Y,
}

impl Axis {
    pub fn both() -> impl Iterator<Item = Self> {
        use Axis::*;
        [X, Y].iter().cloned()
    }

    pub fn for_both<F>(f: F)
    where
        Self: Sized,
        F: FnMut(Self),
    {
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

    pub fn from_slices(x: &[i32], y: &[i32]) -> Vec<Self> {
        assert_eq!(x.len(), y.len());

        let mut coordinates = vec![];
        for (&x, &y) in x.iter().zip(y) {
            coordinates.push(Self::new(x, y));
        }

        coordinates
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
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

    pub fn clamp_to_rectangle(self, low: Self, high: Self) -> Self {
        let mut res = self;
        Axis::for_both(|a| res[a] = low[a].max(high[a].min(res[a])));

        res
    }

    pub fn shrink_extent(self, origin: Self, extent: Self) -> Self {
        self.clamp_to_rectangle(origin, origin + extent - 1)
    }

    pub fn rectilinear_distance_to(self, other: Self) -> i32 {
        let mut distance = 0;
        Axis::for_both(|a| distance += (other[a] - self[a]).abs());

        distance
    }

    pub fn straight_line_distance_to(self, other: Self) -> f32 {
        let mut distance = 0.;
        Axis::for_both(|a| distance += ((other[a] - self[a]) as f32).powi(2));

        distance.sqrt()
    }

    pub fn xy_difference(self, other: Self) -> i32 {
        // Alex: I have no idea what this calculation from the original game is supposed to be.
        // x distance minus y distance means what? Originally found in `Map::CalculateFlow`
        (self.x - other.x).abs() - (self.y - other.y).abs()
    }

    pub fn direction_to(self, other: Self) -> Direction {
        use Direction::*;
        use Ordering::*;
        match (other.x.cmp(&self.x), other.y.cmp(&self.y)) {
            (Equal, Less) => North,
            (Greater, Less) => NorthEast,
            (Greater, Equal) => East,
            (Greater, Greater) => SouthEast,
            (Equal, Greater) => South,
            (Less, Greater) => SouthWest,
            (Less, Equal) => West,
            (Less, Less) => NorthWest,
            (Equal, Equal) => None,
        }
    }

    pub fn direction_to_rounded(self, other: Self) -> Direction {
        use Direction::*;

        let x = other.x - self.x;
        let y = other.y - self.y;
        if x == 0 && y == 0 {
            return None;
        }

        let x = f64::from(x);
        let y = -f64::from(y);
        let angle = y.atan2(x);
        let ordinal = (8. * angle / (2. * std::f64::consts::PI) + 8.).round() as usize % 8;
        match ordinal {
            0 => East,
            1 => NorthEast,
            2 => North,
            3 => NorthWest,
            4 => West,
            5 => SouthWest,
            6 => South,
            7 => SouthEast,
            _ => unreachable!(),
        }
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from(tuple: (i32, i32)) -> Self {
        Self::new(tuple.0, tuple.1)
    }
}

impl From<Coordinate> for (i32, i32) {
    fn from(coordinate: Coordinate) -> Self {
        (coordinate.x, coordinate.y)
    }
}

impl From<Coordinate> for Position {
    fn from(coordinate: Coordinate) -> Self {
        Self::new(coordinate.x, coordinate.y)
    }
}

impl From<Size> for Coordinate {
    fn from(size: Size) -> Self {
        Self::new(size.width, size.height)
    }
}

impl From<Direction> for Coordinate {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::North => Self::new(0, -1),
            Direction::NorthEast => Self::new(1, -1),
            Direction::East => Self::new(1, 0),
            Direction::SouthEast => Self::new(1, 1),
            Direction::South => Self::new(0, 1),
            Direction::SouthWest => Self::new(-1, 1),
            Direction::West => Self::new(-1, 0),
            Direction::NorthWest => Self::new(-1, -1),
            Direction::None => Self::new(0, 0),
        }
    }
}

impl Index<Axis> for Coordinate {
    type Output = i32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
        }
    }
}

impl IndexMut<Axis> for Coordinate {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }
}

impl Add<i32> for Coordinate {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
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
        Self {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
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
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<i32> for Coordinate {
    fn div_assign(&mut self, rhs: i32) {
        Axis::both().for_each(|a| self[a] /= rhs);
    }
}

#[cfg(test)]
mod tests {
    use crate::coordinate::{Coordinate, Direction};

    #[test]
    fn distances() {
        let o = Coordinate::ORIGIN;
        let pythagoras = Coordinate::new(3, 4);

        assert_eq!(7, o.rectilinear_distance_to(pythagoras));
        assert_eq!(5., o.straight_line_distance_to(pythagoras));
    }

    #[test]
    fn directions() {
        let o = Coordinate::ORIGIN;
        let mut u = Coordinate::new(1, 0);

        assert_eq!(o.direction_to(u), Direction::East);
        u.y = 1;
        assert_eq!(o.direction_to(u), Direction::SouthEast);
        u.y = -1;
        assert_eq!(o.direction_to(u), Direction::NorthEast);
        u.x = 0;
        assert_eq!(o.direction_to(u), Direction::North);
        u.x = -1;
        assert_eq!(o.direction_to(u), Direction::NorthWest);
        u.y = 0;
        assert_eq!(o.direction_to(u), Direction::West);
        u.y = 1;
        assert_eq!(o.direction_to(u), Direction::SouthWest);
        u.x = 0;
        assert_eq!(o.direction_to(u), Direction::South);
        u.y = 0;
        assert_eq!(o.direction_to(u), Direction::None);
    }
}
