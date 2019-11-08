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

mod button;
mod checkbox;
mod dialog;
pub mod drawable;
mod label;
mod message_box;
pub mod originals;
mod textbox;
mod ui_container;
pub mod update_result;

pub use button::*;
pub use checkbox::*;
pub use dialog::*;
pub use label::*;
pub use message_box::*;
pub use textbox::*;
pub use ui_container::*;

// TODO: Scrollable

//pub struct Panel {
//    position: Position,
//    size: Size,
//    visibility_fn: Option<Box<dyn Fn() -> bool>>,
//}
//
//impl Panel {
//    pub fn new(size: Size) -> Self {
//        Self::new_with_position(Position { x: 0, y: 0 }, size)
//    }
//
//    pub fn new_with_position(position: Position, size: Size) -> Self {
//        Self { position, size, visibility_fn: None }
//    }
//}
//
//drawable_prerequisites_impl!(Panel);
//
//impl Drawable for Panel {
//    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole) {
//        unimplemented!()
//    }
//
//    fn visible(&self) -> bool {
//        unimplemented!()
//    }
//}
