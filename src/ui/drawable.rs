/*
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
use crate::ui::update_result::{HitResult, UpdateResult};
use crate::ui::{Position, Size};
use crate::util::SafeConsole;

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
        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::drawable::Positioned for $ty< $( $N ),* > {
            fn position(&self) -> $crate::ui::Position {
                self $(.$delegate)?.position
            }
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::drawable::Sized for $ty< $( $N ),* > {
            fn size(&self) -> $crate::ui::Size {
                self $(.$delegate)?.size
            }
        }

        impl< $( $N $(: $b0 $(+$b)* )? ),* > $crate::ui::drawable::VisibilityFn for $ty< $( $N ),* > {
            fn visibility_fn(&self) -> Option<&dyn Fn() -> bool> {
                self $(.$delegate)?.visibility_fn.as_ref().map(|bf| bf.as_ref())
            }
        }
    };
}

pub trait Drawable: Positioned + Sized + VisibilityFn {
    fn draw(&self, relative_position: Position, console: &mut dyn SafeConsole);
    fn update(&mut self, relative_position: Position, input: Input) -> UpdateResult {
        let rectangle = relative_position + self.position() + self.size();
        if rectangle.contains_position(input.mouse_event.character_position) {
            UpdateResult::hit(HitResult::Hit)
        } else {
            UpdateResult::hit(HitResult::NoHit)
        }
    }

    fn visible(&self) -> bool {
        let visibility = self.visibility_fn();
        visibility.is_none() || (visibility.as_ref().unwrap())()
    }
    // TODO: tooltip()
}
