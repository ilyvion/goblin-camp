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

use tcod::console::Root;
use snafu::{Snafu, ResultExt};
use crate::game::game_state::{GameState, GameStateChange};
use crate::game::game_state::main_menu::MainMenu;

pub mod game_state;

#[derive(Debug, Snafu, Eq, PartialEq)]
pub enum Error {
    GameStateError { source: game_state::Error },
    EndGame,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Game {
    root: Root,
    game_states: Vec<Box<dyn GameState>>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    pub fn new() -> Self {
        let char_size = tcod::system::get_char_size();
        let root = Root::initializer()
            .size(640 / char_size.0, 480 / char_size.1) // TODO: Base on config/settings
            .title("Goblin Camp")
            .init();

        Self { root, game_states: vec![MainMenu::game_state()] }
    }

    pub fn run(&mut self) -> Result<()> {
        while !self.root.window_closed() {
            for e in tcod::input::events() {
                dbg!(e.1);
            }

            let current_game_state_index = self.game_states.len();
            if current_game_state_index == 0 {
                return Ok(());
            }
            let current_game_state = self.game_states.get_mut(current_game_state_index - 1).unwrap();

            let game_ref = GameRef {
                root: &mut self.root
            };
            let game_state_change = current_game_state.handle(game_ref).context(GameStateError)?;
            match game_state_change {
                GameStateChange::Replace(next_game_state) => {
                    self.game_states.clear();
                    self.game_states.push(next_game_state);
                }
                GameStateChange::Push(next_game_state) => self.game_states.push(next_game_state),
                GameStateChange::Pop => { self.game_states.pop(); }
                GameStateChange::NoOp => (),
                GameStateChange::EndGame => self.game_states.clear(),
            }

            // Handling user input
            // Updating the gamestate
            // Rendering the results
        }

        Ok(())
    }
}

pub struct GameRef<'g> {
    pub root: &'g mut Root
}
