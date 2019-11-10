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
use crate::game::Input;
use crate::ui::drawable::Drawable;
use crate::ui::originals::{HoldingOriginals, IndexedOriginal};
use crate::ui::update_result::{HitResult, UpdateResult};
use crate::util::SafeConsole;
use crate::{drawable_prerequisites_impl, indexed_original_impl};
use tcod::colors;

pub struct UiContainer {
    position: Position,
    size: Size,
    visibility_fn: Option<Box<dyn Fn() -> bool>>,
    components: Vec<Box<dyn Drawable>>,
}

impl UiContainer {
    pub fn new_with_components<I: Iterator<Item = Box<dyn Drawable>>>(
        components: I,
        position: Position,
        size: Size,
    ) -> Self {
        Self {
            position,
            size,
            components: components.collect(),
            visibility_fn: None,
        }
    }

    pub fn new(position: Position, size: Size) -> Self {
        Self::new_with_components(std::iter::empty(), position, size)
    }

    pub fn add_component<C: Drawable + 'static>(&mut self, component: C) -> IndexedOriginal<C> {
        self.components.push(Box::new(component));
        let components_len = self.components.len();
        IndexedOriginal::new(components_len - 1)
    }
}

drawable_prerequisites_impl!(UiContainer);

impl Drawable for UiContainer {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
        for component in &self.components {
            if component.visible() {
                component.draw(relative_position + self.position, console);
                console.set_default_background(colors::BLACK);
            }
        }
    }

    fn update(&mut self, relative_position: Position, input: Input) -> UpdateResult {
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
            UpdateResult::hit(HitResult::Hit)
        } else {
            UpdateResult::hit(HitResult::NoHit)
        }
    }
}

impl HoldingOriginals<usize> for UiContainer {
    fn get_component(&mut self, token: &usize) -> &mut dyn Drawable {
        #[allow(unsafe_code)]
        unsafe { self.components.get_unchecked_mut(*token) }.as_mut()
    }
}

indexed_original_impl!(UiContainer, ui_container);
