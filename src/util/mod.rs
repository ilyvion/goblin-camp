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

mod safe_console;
pub mod tcod;

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
        match *self {
            None => *self = f(),
            _ => (),
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
