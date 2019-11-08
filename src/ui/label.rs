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
use crate::ui::drawable::Drawable;
use crate::util::SafeConsole;
use crate::{drawable_prerequisites_impl, indexed_original_impl};
use tcod::{colors, TextAlignment};

pub struct Label {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,

    text: String,
    alignment: TextAlignment,
    color: colors::Color,
}

impl Label {
    pub fn new<S: AsRef<str>>(text: S, position: Position) -> Self {
        Self::new_with_alignment(text, position, TextAlignment::Left)
    }

    pub fn new_with_alignment<S: AsRef<str>>(
        text: S,
        position: Position,
        alignment: TextAlignment,
    ) -> Self {
        Self {
            position,
            size: Size::new(0, 1),
            visibility_fn: None,

            text: text.as_ref().to_string(),
            alignment,
            color: colors::WHITE,
        }
    }

    pub fn set_color(&mut self, color: colors::Color) {
        self.color = color;
    }
}

drawable_prerequisites_impl!(Label);

impl Drawable for Label {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
        console.set_alignment(self.alignment);
        console.set_default_foreground(self.color);
        console.print(relative_position + self.position, &self.text);
    }
}

indexed_original_impl!(Label, label);
