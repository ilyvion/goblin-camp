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

use crate::ui::drawable::Drawable;
use crate::ui::{Position, Size};
use crate::util::SafeConsole;
use crate::{drawable_prerequisites_impl, indexed_original_impl};
use tcod::colors;

/// WIP
pub struct CheckBox {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,

    checked: bool,
    text: String,
    color: colors::Color,
    checked_color: colors::Color,
}

impl CheckBox {
    const EMPTY_CHECKBOX: &'static str = "à";
    const CHECKED_CHECKBOX: &'static str = "á";

    pub fn new<S: AsRef<str>>(
        text: S,
        position: Position,
        checked: bool,
        color: colors::Color,
        checked_color: colors::Color,
    ) -> Self {
        Self {
            position,
            size: Size::new(0, 1),
            visibility_fn: None,

            checked,
            text: text.as_ref().to_string(),
            color,
            checked_color,
        }
    }
}

drawable_prerequisites_impl!(CheckBox);

impl Drawable for CheckBox {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
        console.set_default_foreground(if self.checked {
            self.checked_color
        } else {
            self.color
        });

        console.print(
            relative_position + self.position,
            if !self.checked {
                Self::EMPTY_CHECKBOX
            } else {
                Self::CHECKED_CHECKBOX
            },
        );
        console.print(relative_position + self.position + (2, 0), &self.text);
    }
}

indexed_original_impl!(CheckBox, checkbox);
