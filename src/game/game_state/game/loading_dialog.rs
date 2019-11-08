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

use crate::game::game_state::{GameState, GameStateChange, GameStateResult, GameStateUpdateResult};
use crate::game::GameRef;
use std::borrow::Cow;
use std::error::Error;
use tcod::{colors, BackgroundFlag, Console, TextAlignment};

const LOADING: [&str; 4] = [
    "\\ Loading...",
    "| Loading...",
    "/ Loading...",
    "- Loading...",
];

#[derive(Default)]
pub struct LoadingDialog {
    current_spinner: usize,
    x: i32,
    y: i32,
}

impl LoadingDialog {
    pub fn game_state_change() -> GameStateChange {
        GameStateChange::Push(Self::game_state())
    }

    pub fn game_state() -> Box<dyn GameState> {
        Box::new(Self::default())
    }
}

impl GameState for LoadingDialog {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Loading dialog")
    }

    fn activate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        self.x = game_ref.root.width();
        self.y = game_ref.root.height();

        Ok(())
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        self.current_spinner += 1;

        if game_ref
            .background_update_messages
            .iter()
            .any(|m| m == "DoneLoading")
        {
            Ok(GameStateChange::Pop)
        } else {
            Ok(GameStateChange::None)
        }
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        game_ref.root.set_default_foreground(colors::WHITE);
        game_ref.root.set_default_background(colors::BLACK);
        game_ref.root.set_alignment(TextAlignment::Center);
        game_ref.root.rect(
            0,
            0,
            game_ref.root.width(),
            game_ref.root.height(),
            true,
            BackgroundFlag::Set,
        );
        game_ref.root.print(
            self.x / 2,
            self.y / 2,
            LOADING[self.current_spinner % LOADING.len()],
        );

        Ok(())
    }
}
