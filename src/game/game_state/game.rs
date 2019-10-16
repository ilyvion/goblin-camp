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

use crate::game::game_state::{GameState, GameStateChange, GameStateError};
use crate::game::GameRef;
use crate::ui::MessageBox;
use std::borrow::Cow;

pub struct ConfirmNewGame;

impl ConfirmNewGame {
    pub fn game_state() -> GameStateChange {
        GameStateChange::Push(Box::new(ConfirmNewGame))
    }
}

impl GameState for ConfirmNewGame {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Confirm new game")
    }

    fn update(&mut self, game_ref: &mut GameRef) -> Result<GameStateChange, GameStateError> {
        game_ref.is_running = true;
        if game_ref.is_running {
            // TODO: Find a way to generalize these message boxes so that
            Ok(GameStateChange::PopPush(MessageBox::game_state(
                game_ref,
                "A game is already running, are you sure you want  to start a new one?",
                "Yes",
                Box::new(|| GameStateChange::EndGame),
                Some("No"),
                Some(Box::new(|| GameStateChange::Pop)),
            )))
        } else {
            // TODO: Don't just Pop, but start a new game
            Ok(GameStateChange::Pop)
        }
    }

    fn draw(&mut self, _: &mut GameRef) -> Result<(), GameStateError> {
        Ok(())
    }
}
