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

use crate::util::SafeConsole;
use std::ops::Add;

mod button;
mod dialog;
mod label;
mod message_box;
mod ui_container;

pub use button::*;
pub use dialog::*;
pub use label::*;
pub use message_box::*;
pub use ui_container::*;

use crate::game::Input;
use std::any::Any;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub const ORIGIN: Position = Position { x: 0, y: 0 };

    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<Size> for Position {
    type Output = Rectangle;

    fn add(self, rhs: Size) -> Self::Output {
        Rectangle {
            position: self,
            size: rhs,
        }
    }
}

impl Add<(i32, i32)> for Position {
    type Output = Position;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

impl Size {
    pub fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

impl Add<(i32, i32)> for Size {
    type Output = Size;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            width: self.width + rhs.0,
            height: self.height + rhs.1,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Rectangle {
    pub position: Position,
    pub size: Size,
}

impl Rectangle {
    fn contains_position(&self, position: Position) -> bool {
        position.x >= self.position.x
            && position.x < self.position.x + self.size.width
            && position.y >= self.position.y
            && position.y < self.position.y + self.size.height
    }
}

#[derive(Eq, PartialEq)]
pub enum HitResult {
    Hit,
    NoHit,
}

pub enum RespondKind {
    Key(char),
    Mouse(Position),
}

pub struct MenuResult {
    hit: HitResult,
    kind: Option<RespondKind>,
    _dismiss: bool,
    data: Option<Box<dyn Any>>,
}

impl MenuResult {
    pub fn new(
        hit: HitResult,
        kind: Option<RespondKind>,
        dismiss: bool,
        data: Option<Box<dyn Any>>,
    ) -> Self {
        Self {
            hit,
            kind,
            _dismiss: dismiss,
            data,
        }
    }

    pub fn hit(hit: HitResult) -> Self {
        Self::new(hit, None, false, None)
    }
}

pub trait Positioned {
    fn position(&self) -> Position;
}

pub trait Sized {
    fn size(&self) -> Size;
}

pub trait VisibilityFn {
    fn visibility_fn(&self) -> Option<&dyn Fn() -> bool>;
}

#[macro_export]
macro_rules! drawable_prerequisites_impl {
    ($ty:ident $(, $delegate:ident)?) => {
        $crate::drawable_prerequisites_impl!($ty<> $(,$delegate)?);
    };
    ($ty:ident < $( $N:ident $(: $b0:ident $(+$b:ident)* )? ),* > $(, $delegate:ident)?) => {
        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::Positioned for $ty< $( $N ),* > {
            fn position(&self) -> $crate::ui::Position {
                self $(.$delegate)?.position
            }
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::Sized for $ty< $( $N ),* > {
            fn size(&self) -> $crate::ui::Size {
                self $(.$delegate)?.size
            }
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::VisibilityFn for $ty< $( $N ),* > {
            fn visibility_fn(&self) -> Option<&dyn Fn() -> bool> {
                self $(.$delegate)?.visibility_fn.as_ref().map(|bf| bf.as_ref())
            }
        }
    };
}

pub trait Drawable: Positioned + Sized + VisibilityFn {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole);
    fn update(&mut self, relative_position: Position, input: Input) -> MenuResult {
        let rectangle = relative_position + self.position() + self.size();
        if rectangle.contains_position(input.mouse_event.character_position) {
            MenuResult::hit(HitResult::Hit)
        } else {
            MenuResult::hit(HitResult::NoHit)
        }
    }

    fn visible(&self) -> bool {
        let visibility = self.visibility_fn();
        visibility.is_none() || (visibility.as_ref().unwrap())()
    }
    // TODO: tooltip()
}

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
