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

use crate::ui::Position;
use std::any::Any;

#[derive(Eq, PartialEq)]
pub enum HitResult {
    Hit,
    NoHit,
}

pub enum RespondKind {
    Key(char),
    Mouse(Position),
}

pub struct UpdateResult {
    pub hit: HitResult,
    pub kind: Option<RespondKind>,
    pub dismiss: bool,
    pub data: Option<Box<dyn Any>>,
}

impl UpdateResult {
    pub fn new(
        hit: HitResult,
        kind: Option<RespondKind>,
        dismiss: bool,
        data: Option<Box<dyn Any>>,
    ) -> Self {
        Self {
            hit,
            kind,
            dismiss,
            data,
        }
    }

    pub fn hit(hit: HitResult) -> Self {
        Self::new(hit, None, false, None)
    }
}
