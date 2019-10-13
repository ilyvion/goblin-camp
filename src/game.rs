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
use slog::{o, debug};
use crate::game::game_state::{GameState, GameStateChange};
use crate::game::game_state::main_menu::MainMenu;
use crate::Config;

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
    logger: slog::Logger,
    config: Config,
}

impl Game {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const NAME: &'static str = "Goblin Camp";

    pub fn new(parent_logger: slog::Logger, config: Config) -> Self {
        let logger = parent_logger.new(o!());
        let method_logger = logger.new(o!("Method" => "Game::new"));

        let char_size = tcod::system::get_char_size();
        debug!(method_logger, "Character size: ({}, {})", char_size.0, char_size.1);
        debug!(method_logger, "Window size: ({}, {})", config.window_width as i32 / char_size.0, config.window_height as i32 / char_size.1);

        let root = Root::initializer()
            .size(config.window_width as i32 / char_size.0, config.window_height as i32 / char_size.1)
            .title(Game::NAME)
            .init();

        Self { root, game_states: vec![MainMenu::game_state()], logger, config }
    }

    pub fn run(&mut self) -> Result<()> {
        let method_logger = self.logger.new(o!("Method" => "Game::run"));
        while !self.root.window_closed() {
            let current_game_state_index = self.game_states.len();
            debug!(method_logger, "Current game states size: {}", current_game_state_index);

            if current_game_state_index == 0 {
                debug!(method_logger, "Out of game states; returning");
                return Ok(());
            }
            let current_game_state = self.game_states.get_mut(current_game_state_index - 1).unwrap();

            let game_ref = GameRef {
                root: &mut self.root,
                config: &self.config,
                logger: &self.logger,
                is_running: false,
            };

            let game_state_change = current_game_state.handle(game_ref).context(GameStateError)?;
            match game_state_change {
                GameStateChange::Replace(next_game_state) => {
                    debug!(method_logger, "Game state change: Replace");
                    self.game_states.clear();
                    self.game_states.push(next_game_state);
                }
                GameStateChange::Push(next_game_state) => {
                    debug!(method_logger, "Game state change: Push");
                    self.game_states.push(next_game_state)
                },
                GameStateChange::Pop => {
                    debug!(method_logger, "Game state change: Pop");
                    self.game_states.pop();
                },
                GameStateChange::EndGame => {
                    debug!(method_logger, "Game state change: EndGame");
                    self.game_states.clear()
                },
                GameStateChange::NoOp => {
                    debug!(method_logger, "Game state change: NoOp");
                },
            }

            // Handling user input
            // Updating the gamestate
            // Rendering the results
        }

        Ok(())
    }
}

pub struct GameRef<'g> {
    pub root: &'g mut Root,
    pub config: &'g Config,
    pub logger: &'g slog::Logger,
    pub is_running: bool,
}
