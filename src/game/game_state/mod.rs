/*
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

use crate::game::GameRef;
use snafu::Snafu;
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result as FormatterResult};

#[derive(Debug, Snafu, Eq, PartialEq)]
pub enum GameStateError {}

pub type GameStateResult = std::result::Result<(), GameStateError>;
pub type GameStateUpdateResult = std::result::Result<GameStateChange, GameStateError>;

pub enum GameStateChange {
    Replace(Box<dyn GameState>),
    Push(Box<dyn GameState>),
    PopPush(Box<dyn GameState>),
    Pop,
    NoOp,
    EndGame,
}

/// Represents a state the game can be in, like main menu, game, pause screen, message box,
/// etc. These can be stacked, and will be told when they are active (i.e. on top) or not,
/// and they can behave accordingly.
pub trait GameState {
    /// Provide the name of this `GameState`.
    fn name(&self) -> Cow<'_, str>;

    /// Called once each time this `GameState` becomes the active one.
    #[allow(unused_variables)]
    fn activate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        Ok(())
    }

    /// Called once each time this `GameState` stops being the active one.
    #[allow(unused_variables)]
    fn deactivate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        Ok(())
    }

    /// When this `GameState` is underneath one or more other `GameState`s, this method will be
    /// called. These are called in the order they are in the game state stack, from bottom to top.

    #[allow(unused_variables)]
    fn background_update(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        Ok(())
    }

    /// Called once each game tick; used to update game state.
    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult;

    /// When this `GameState` is underneath one or more other `GameState`s, this method will be
    /// called. These are called in the order they are in the game state stack, from bottom to top.
    fn background_draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        self.draw(game_ref)
    }

    /// Called once each game tick; used to draw to the screen. These are called in the order they
    /// are in the game state stack, from bottom to top.
    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult;
}

impl Display for dyn GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatterResult {
        write!(f, "{}", self.name())
    }
}

pub mod main_menu;

pub mod game;
