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

mod array2d;
mod safe_console;
pub mod tcod;

pub use array2d::*;
pub use safe_console::*;
use std::mem;

pub trait Flip {
    fn flip(&mut self);
}

impl Flip for bool {
    fn flip(&mut self) {
        mem::replace(self, !*self);
    }
}

pub trait OptionExt<T> {
    fn get_or_maybe_insert(&mut self, v: Option<T>) -> Option<&mut T> {
        self.get_or_maybe_insert_with(|| v)
    }

    fn get_or_maybe_insert_with<F: FnOnce() -> Option<T>>(&mut self, f: F) -> Option<&mut T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn get_or_maybe_insert_with<F: FnOnce() -> Option<T>>(&mut self, f: F) -> Option<&mut T> {
        if self.is_none() {
            *self = f();
        }

        self.as_mut()
    }
}

/// Takes a slice of `values`, looking at them two at a time, and if the given `value` is smaller
/// or equal to the second value, set it to the first value.
pub fn find_largest_fit<T: PartialOrd + Copy>(value: &mut T, values: &[T]) {
    let mut value_set = false;
    for values in values.windows(2) {
        if *value <= values[1] {
            *value = values[0];
            value_set = true;
            break;
        }
    }
    if !value_set {
        *value = values[values.len() - 1];
    }
}

/// Compares the values in a slice two at a time, and based on whether the left or the right value
/// is bigger, puts `left_winner` or `right_winner` into the result vector. Should the values be the
/// same, the `tie_breaker` decides who wins.
/// `false` picks `left_winner` and `true` picks `right_winner`.
pub fn compare_and_pick<T: PartialOrd, R: Copy, F: FnMut() -> bool>(
    data: &[T],
    left_winner: R,
    right_winner: R,
    mut tie_breaker: F,
) -> Vec<R> {
    let mut results = vec![];
    for values in data.windows(2) {
        if values[0] < values[1] {
            results.push(right_winner);
        } else if values[0] > values[1] {
            results.push(left_winner);
        } else if tie_breaker() {
            results.push(right_winner);
        } else {
            results.push(left_winner);
        }
    }

    results
}

pub fn dual_map<T: Copy, R, F: FnMut(T, T) -> R>(data: &[T], mut map: F) -> Vec<R> {
    let mut results = vec![];
    for values in data.windows(2) {
        results.push(map(values[0], values[1]));
    }

    results
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}
