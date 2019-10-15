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
use crate::ui::{Position, Size, Drawable, MenuResult, HitResult};
use crate::util::SafeConsole;
use crate::game::Input;

pub struct UiContainer {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,
    components: Vec<Box<dyn Drawable>>,
}

impl UiContainer {
    pub fn new<I: Iterator<Item=Box<dyn Drawable>>>(components: I, position: Position, size: Size) -> Self {
        Self { position, size, components: components.collect(), visibility_fn: None }
    }

    pub fn add_component(&mut self, component: Box<dyn Drawable>) {
        self.components.push(component)
    }
}

drawable_prerequisites_impl!(UiContainer);

impl Drawable for UiContainer {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
        for component in &self.components {
            if component.visible() {
                component.draw(relative_position + self.position, console);
            }
        }
    }

    fn update(&mut self, relative_position: Position, input: Input) -> MenuResult {
        for component in &mut self.components {
            if component.visible() {
                let result = component.update(relative_position + self.position, input.clone());
                if result.hit == HitResult::Hit {
                    return result;
                }
            }
        }

        let rectangle = self.position + self.size;
        if rectangle.contains_position(input.mouse_event.character_position) {
            MenuResult::hit(HitResult::Hit)
        } else {
            MenuResult::hit(HitResult::NoHit)
        }
    }
}
