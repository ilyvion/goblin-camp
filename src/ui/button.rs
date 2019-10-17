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

use crate::game::Input;
use crate::ui::drawable::Drawable;
use crate::ui::menu_result::{HitResult, MenuResult, RespondKind};
use crate::ui::{Position, Size};
use crate::util::SafeConsole;
use crate::{drawable_prerequisites_impl, indexed_original_impl};
use std::any::Any;
use tcod::{colors, BackgroundFlag, TextAlignment};

pub struct Button {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,

    text: String,
    selected: bool,
    shortcut: Option<char>,
    dismiss: bool,
}

impl Button {
    pub fn new_with_defaults<S: AsRef<str>>(text: S, position: Position, width: i32) -> Self {
        Self::new(text, position, width, None, false)
    }

    pub fn new<S: AsRef<str>>(
        text: S,
        position: Position,
        width: i32,
        shortcut: Option<char>,
        dismiss: bool,
    ) -> Self {
        Self {
            position,
            size: Size::new(width, 0),
            visibility_fn: None,

            text: text.as_ref().to_string(),
            selected: false,
            shortcut,
            dismiss,
        }
    }
}

drawable_prerequisites_impl!(Button);

impl Drawable for Button {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
        console.set_background_flag(BackgroundFlag::Set);
        if self.selected {
            console.set_default_foreground(colors::BLACK);
            console.set_default_background(colors::WHITE);
        } else {
            console.set_default_foreground(colors::WHITE);
            console.set_default_background(colors::BLACK);
        }
        console.set_alignment(TextAlignment::Center);
        console.print_frame(
            relative_position + self.position,
            self.size + (0, 3),
            true,
            BackgroundFlag::Default,
            None,
        );
        console.print(
            relative_position + self.position + (self.size.width / 2, 1),
            &self.text,
        );
    }

    fn update(&mut self, relative_position: Position, input: Input) -> MenuResult {
        if let Some(shortcut) = self.shortcut {
            if input.key_event.raw.printable == shortcut {
                // TODO: Also support keycode?
                return MenuResult::new(
                    HitResult::Hit,
                    Some(RespondKind::Key(shortcut)),
                    self.dismiss,
                    Some(Box::new(self.text.clone())),
                );
            }
        }

        let size = Size::new(self.size.width, 3);
        let rectangle = relative_position + self.position + size;
        if rectangle.contains_position(input.mouse_event.character_position) {
            self.selected = true;
            let response = if input.mouse_event.clicked {
                Some(RespondKind::Mouse(input.mouse_event.character_position))
            } else {
                None
            };
            let data: Option<Box<dyn Any>> = if response.is_some() {
                Some(Box::new(self.text.clone()))
            } else {
                None
            };
            MenuResult::new(HitResult::Hit, response, self.dismiss, data)
        } else {
            self.selected = false;

            MenuResult::hit(HitResult::NoHit)
        }
    }
}

indexed_original_impl!(Button, button);
