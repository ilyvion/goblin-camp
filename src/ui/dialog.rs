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

use crate::drawable_prerequisites_impl;
use crate::ui::{Position, Drawable, Size, Positioned, Sized, MenuResult};
use crate::game::{GameRef, Input};
use tcod::{Console, BackgroundFlag};
use crate::util::SafeConsole;

pub struct Dialog<D: Drawable> {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,

    title: Option<String>,
    contents: D,
}

impl<D: Drawable> Dialog<D> {
    pub fn new(game_ref: &mut GameRef, contents: D, title: Option<String>, size: Size) -> Self {
        Self {
            position: Position::new((game_ref.root.width() - size.width) / 2, (game_ref.root.height() - size.height) / 2),
            size,
            visibility_fn: None,

            title,
            contents,
        }
    }
}

drawable_prerequisites_impl!(Dialog<D: Drawable>);

impl<D: Drawable> Drawable for Dialog<D> {
    fn draw(&self, _: Position, console: &mut dyn SafeConsole) {
        console.print_frame(self.position(), self.size(), true, BackgroundFlag::Set, self.title.as_ref().map(|s| s.as_str()));
        self.contents.draw(self.position(), console);
    }

    fn update(&mut self, _: Position, input: Input) -> MenuResult {
        self.contents.update(self.position(), input)
    }
}