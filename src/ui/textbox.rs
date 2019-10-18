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
use tcod::{colors, BackgroundFlag};

/// WIP
pub struct TextBox {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,

    text: String,
    color: colors::Color,
    background_color: colors::Color,
}

impl TextBox {
    pub fn new<S: AsRef<str>>(text: S, position: Position, width: i32) -> Self {
        Self::new_with_alignment(text, position, width)
    }

    pub fn new_with_alignment<S: AsRef<str>>(text: S, position: Position, width: i32) -> Self {
        Self {
            position,
            size: Size::new(width, 1),
            visibility_fn: None,

            text: text.as_ref().to_string(),
            color: colors::WHITE,
            background_color: colors::DARK_GREY,
        }
    }

    pub fn set_color(&mut self, color: colors::Color) {
        self.color = color;
    }

    pub fn set_background_color(&mut self, color: colors::Color) {
        self.background_color = color;
    }

    pub fn set_text<S: AsRef<str>>(&mut self, text: S) {
        self.text = text.as_ref().to_string();
    }
}

drawable_prerequisites_impl!(TextBox);

impl Drawable for TextBox {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
        console.set_default_background(self.background_color);
        console.set_default_foreground(self.color);
        console.rect(
            relative_position + self.position + self.size,
            true,
            BackgroundFlag::Default,
        );
        console.print(relative_position + self.position, &self.text);
    }
}

indexed_original_impl!(TextBox, text_box);
