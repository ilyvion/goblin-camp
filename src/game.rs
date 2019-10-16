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

use crate::data::Data;
use crate::game::game_state::main_menu::MainMenu;
use crate::game::game_state::{GameState, GameStateChange};
use crate::ui::Position;
use crate::Config;
use slog::{debug, o, trace};
use snafu::{ResultExt, Snafu};
use tcod::console::Root;
use tcod::input::{Key, Mouse};
use tcod::{colors, input, Console};

pub mod game_state;

#[derive(Debug, Snafu)]
pub enum Error {
    GameStateError { source: game_state::GameStateError },
    EndGame,
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct Game {
    root: Root,
    game_states: Vec<Box<dyn GameState>>,
    logger: slog::Logger,
    config: Config,
    data: Data,
    previous_mouse: Mouse,
}

impl Game {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const NAME: &'static str = "Goblin Camp Revival";

    pub fn new(parent_logger: slog::Logger, config: Config, data: Data) -> Self {
        let logger = parent_logger.new(o!());
        let method_logger = logger.new(o!("Method" => "Game::new"));

        let size = if data.settings.fullscreen {
            tcod::system::get_current_resolution()
        } else {
            (
                data.settings.resolution_x as i32,
                data.settings.resolution_y as i32,
            )
        };

        let char_size = tcod::system::get_char_size();
        debug!(
            method_logger,
            "Character size: ({}, {})", char_size.0, char_size.1
        );
        debug!(method_logger, "Window size: ({}, {})", size.0, size.1);
        debug!(
            method_logger,
            "Window characters: ({}, {})",
            size.0 / char_size.0,
            size.1 / char_size.1
        );

        let root = Root::initializer()
            .size(size.0 / char_size.0, size.1 / char_size.1)
            .fullscreen(data.settings.fullscreen)
            .title(Game::NAME)
            .renderer(data.settings.renderer.into())
            .init();
        tcod::input::show_cursor(true);

        Self {
            root,
            game_states: vec![MainMenu::game_state()],
            logger,
            config,
            data,
            previous_mouse: Mouse::default(),
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let method_logger = self.logger.new(o!("Method" => "Game::run"));
        let mut game_state_changed = true;
        while !self.root.window_closed() {
            let current_game_state_length = self.game_states.len();
            trace!(
                method_logger,
                "Current game states size: {}",
                current_game_state_length
            );

            if current_game_state_length == 0 {
                debug!(method_logger, "Out of game states; returning");
                return Ok(());
            }

            let input = self.handle_input();
            let mut game_ref = GameRef {
                root: &mut self.root,
                config: &self.config,
                logger: &self.logger,
                data: &mut self.data,
                is_running: false,
                game_state_level: current_game_state_length,
                input,
            };

            if game_state_changed {
                let current_game_state = self
                    .game_states
                    .get_mut(current_game_state_length - 1)
                    .unwrap();
                debug!(
                    method_logger,
                    "Calling activate on game state {}",
                    current_game_state.name()
                );
                current_game_state
                    .activate(&mut game_ref)
                    .context(GameStateError)?;
                game_state_changed = false;
            }

            let game_state_change = 'update: loop {
                for i in 0..current_game_state_length {
                    if i != current_game_state_length - 1 {
                        self.game_states
                            .get_mut(i)
                            .unwrap()
                            .background_update(&mut game_ref)
                            .context(GameStateError)?;
                    } else {
                        break 'update self
                            .game_states
                            .get_mut(i)
                            .unwrap()
                            .update(&mut game_ref)
                            .context(GameStateError)?;
                    }
                }
            };

//            // Iterator based alternative. Not much better...
//            let game_state_change = self.game_states.iter_mut().enumerate().map(|(i, state)| {
//                if i != current_game_state_length - 1 {
//                    state.background_update(&mut game_ref).context(GameStateError)?;
//                    Ok(None)
//                } else {
//                    Ok(Some(state.update(&mut game_ref).context(GameStateError)?))
//                }
//            } ).collect::<Result<Vec<_>, _>>()?.into_iter().filter_map(|o| o).next().unwrap();

            game_ref.root.set_default_background(colors::BLACK);
            game_ref.root.set_default_foreground(colors::WHITE);
            game_ref.root.clear();
            'draw: loop {
                for i in 0..current_game_state_length {
                    if i != current_game_state_length - 1 {
                        self.game_states
                            .get_mut(i)
                            .unwrap()
                            .background_draw(&mut game_ref)
                            .context(GameStateError)?;
                    } else {
                        self.game_states
                            .get_mut(i)
                            .unwrap()
                            .draw(&mut game_ref)
                            .context(GameStateError)?;
                        break 'draw;
                    }
                }
            }
            game_ref.root.flush();

            let deactivate = if let GameStateChange::None = &game_state_change {
                false
            } else {
                true
            };
            if deactivate {
                let current_game_state = self
                    .game_states
                    .get_mut(current_game_state_length - 1)
                    .unwrap();
                debug!(
                    method_logger,
                    "Calling deactivate on game state {}",
                    current_game_state.name()
                );
                current_game_state
                    .deactivate(&mut game_ref)
                    .context(GameStateError)?;
                game_state_changed = true;
            }

            match game_state_change {
                GameStateChange::Replace(next_game_state) => {
                    trace!(method_logger, "Game state change: Replace");
                    self.game_states.clear();
                    self.game_states.push(next_game_state);
                }
                GameStateChange::Push(next_game_state) => {
                    trace!(method_logger, "Game state change: Push");
                    self.game_states.push(next_game_state)
                }
                GameStateChange::PopPush(next_game_state) => {
                    trace!(method_logger, "Game state change: PopPush");
                    self.game_states.pop();
                    self.game_states.push(next_game_state)
                }
                GameStateChange::Pop => {
                    trace!(method_logger, "Game state change: Pop");
                    self.game_states.pop();
                }
                GameStateChange::EndGame => {
                    trace!(method_logger, "Game state change: EndGame");
                    self.game_states.clear()
                }
                GameStateChange::None => {
                    trace!(method_logger, "Game state change: None");
                }
            }
        }

        Ok(())
    }

    pub fn handle_input(&mut self) -> Input {
        let mut raw_events = vec![];
        let mut key_event = None;
        let mut mouse_event = None;
        for (flags, event) in tcod::input::events() {
            raw_events.push(event);
            if flags.intersects(input::KEY_RELEASE) && key_event.is_none() {
                if let input::Event::Key(key) = event {
                    key_event = Some(KeyEvent { raw: key });
                }
            } else if flags.intersects(input::MOUSE) && mouse_event.is_none() {
                if let input::Event::Mouse(mouse) = event {
                    mouse_event = Some(mouse.into());
                    self.previous_mouse = mouse;

                    // We have to do this, otherwise multiple click events will trigger
                    self.previous_mouse.lbutton_pressed = false;
                    self.previous_mouse.rbutton_pressed = false;
                    self.previous_mouse.mbutton_pressed = false;
                }
            }
        }

        Input {
            raw_events,
            key_event: key_event.unwrap_or_default(),
            mouse_event: mouse_event.unwrap_or_else(|| self.previous_mouse.into()),
        }
    }
}

#[derive(Clone)]
pub struct Input {
    pub raw_events: Vec<input::Event>,
    pub key_event: KeyEvent,
    pub mouse_event: MouseEvent,
}

#[derive(Copy, Clone, Default)]
pub struct KeyEvent {
    pub raw: Key,
}

#[derive(Copy, Clone, Default)]
pub struct MouseEvent {
    pub raw: Mouse,
    pub character_position: Position,
    pub screen_position: Position,
    pub clicked: bool,
}

impl From<Mouse> for MouseEvent {
    fn from(mouse: Mouse) -> Self {
        Self {
            raw: mouse,
            character_position: Position::new(mouse.cx as i32, mouse.cy as i32),
            screen_position: Position::new(mouse.x as i32, mouse.y as i32),
            clicked: mouse.lbutton_pressed,
        }
    }
}

pub struct GameRef<'g> {
    pub root: &'g mut Root,
    pub config: &'g Config,
    pub logger: &'g slog::Logger,
    pub data: &'g mut Data,
    pub is_running: bool,
    pub game_state_level: usize,
    pub input: Input,
}
