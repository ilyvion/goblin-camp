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

use crate::game::{GameRef, Input};
use crate::ui::drawable::{Drawable, Positioned, Sized};
use crate::ui::menu_result::MenuResult;
use crate::ui::{Position, Size};
use crate::util::SafeConsole;
use crate::{drawable_prerequisites_impl, indexed_original_impl};
use tcod::BackgroundFlag;

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
            position: Position::new(
                (game_ref.root.width() - size.width) / 2,
                (game_ref.root.height() - size.height) / 2,
            ),
            size,
            visibility_fn: None,

            title,
            contents,
        }
    }

    pub fn get_contents(&mut self) -> &mut D {
        &mut self.contents
    }
}

drawable_prerequisites_impl!(Dialog<D: Drawable>);

impl<D: Drawable> Drawable for Dialog<D> {
    fn draw(&self, _: Position, console: &mut dyn SafeConsole) {
        console.print_frame(
            self.position(),
            self.size(),
            true,
            BackgroundFlag::Set,
            self.title.as_ref().map(|s| s.as_str()),
        );
        self.contents.draw(self.position(), console);
    }

    fn update(&mut self, _: Position, input: Input) -> MenuResult {
        self.contents.update(self.position(), input)
    }
}

indexed_original_impl!(Dialog<D: Drawable>, dialog);
