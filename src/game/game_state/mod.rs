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

use snafu::{Snafu};
use crate::game::GameRef;

#[derive(Debug, Snafu, Eq, PartialEq)]
pub enum Error {
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub enum GameStateChange {
    Replace(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    Pop,
    NoOp,
    EndGame
}

pub trait GameState {
    fn handle(&mut self, game_ref: GameRef) -> Result<GameStateChange>;
}

pub mod main_menu;

