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

use serde_derive::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FormatResult};
use std::ops::{Add, DivAssign, Sub};

#[derive(Copy, Clone, Default, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const ORIGIN: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl From<Position> for (i32, i32) {
    fn from(p: Position) -> Self {
        (p.x, p.y)
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Size> for Position {
    type Output = Rectangle;

    fn add(self, rhs: Size) -> Self::Output {
        Rectangle {
            position: self,
            size: rhs,
        }
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

impl DivAssign<i32> for Position {
    fn div_assign(&mut self, rhs: i32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub const fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }

    pub fn area(self) -> i32 {
        self.width * self.height
    }
}

impl From<Size> for (i32, i32) {
    fn from(s: Size) -> Self {
        (s.width, s.height)
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}x{}", self.width, self.height)
    }
}

impl Add<(i32, i32)> for Size {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            width: self.width + rhs.0,
            height: self.height + rhs.1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size,
}

impl Rectangle {
    pub fn contains_position(&self, position: Position) -> bool {
        position.x >= self.position.x
            && position.x < self.position.x + self.size.width
            && position.y >= self.position.y
            && position.y < self.position.y + self.size.height
    }
}
