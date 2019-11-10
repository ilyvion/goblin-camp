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

use crate::data::base::Position;
use crate::data::Data;
use crate::game::game_data::GameData;
use crate::game::game_state::main_menu::MainMenu;
use crate::game::game_state::{GameState, GameStateChange};
use crate::Config;
use slog::{debug, o, trace};
use snafu::{ResultExt, Snafu};
use tcod::console::{Offscreen, Root};
use tcod::input::{Key, Mouse};
use tcod::{colors, input, Console};

pub mod game_data;
pub mod game_state;

#[derive(Debug, Snafu)]
pub enum Error {
    GameStateError { source: game_state::GameStateError },
    EndGame,
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct Game {
    root: Root,
    buffer: Offscreen,
    game_states: Vec<Box<dyn GameState>>,
    logger: slog::Logger,
    config: Config,
    data: Data,
    previous_mouse: Mouse,
    game_data: GameData,
}

impl Game {
    pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    pub const NAME: &'static str = "Goblin Camp Revival";

    pub fn new(parent_logger: &slog::Logger, config: Config, data: Data) -> Self {
        let logger = parent_logger.new(o!());
        let method_logger = logger.new(o!("Method" => "Game::new"));

        let size = if data.settings.display.fullscreen {
            tcod::system::get_current_resolution()
        } else {
            data.settings.display.resolution.into()
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
            .fullscreen(data.settings.display.fullscreen)
            .title(Self::NAME)
            .renderer(data.settings.renderer.into())
            .init();
        tcod::input::show_cursor(true);

        let buffer = Offscreen::new(size.0 / char_size.0, size.1 / char_size.1);

        Self {
            root,
            buffer,
            game_states: vec![MainMenu::game_state()],
            logger,
            config,
            data,
            previous_mouse: Mouse::default(),
            game_data: GameData::new(),
        }
    }

    pub fn run(&mut self) -> Result {
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
                buffer: &mut self.buffer,
                config: &self.config,
                logger: &self.logger,
                data: &mut self.data,
                game_state_level: current_game_state_length,
                input,
                game_data: &mut self.game_data,
                background_update_messages: vec![],
            };

            let mut game_loop_data = GameLoopData {
                parent_logger: &self.logger,
                game_states: &mut self.game_states,
                current_game_state_length,
                game_state_changed: &mut game_state_changed,
                game_ref: &mut game_ref,
            };

            Self::activate_game_state(&mut game_loop_data)?;
            let game_state_change = Self::update_game_states(&mut game_loop_data)?;
            Self::draw_game_states(&mut game_loop_data)?;
            Self::deactivate_game_state(&mut game_loop_data, &game_state_change)?;
            Self::update_game_state(&mut game_loop_data, game_state_change);
        }

        Ok(())
    }

    fn handle_input(&mut self) -> Input {
        let mut raw_events = vec![];
        let mut key_release_event = None;
        let mut key_press_event = None;
        let mut mouse_event = None;
        for (flags, event) in tcod::input::events() {
            raw_events.push(event);
            if flags.intersects(input::KEY_RELEASE) && key_release_event.is_none() {
                if let input::Event::Key(key) = event {
                    key_release_event = Some(KeyEvent { raw: key });
                }
            } else if flags.intersects(input::KEY_PRESS) && key_press_event.is_none() {
                if let input::Event::Key(key) = event {
                    key_press_event = Some(KeyEvent { raw: key });
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
            press_key_event: key_press_event.unwrap_or_default(),
            release_key_event: key_release_event.unwrap_or_default(),
            mouse_event: mouse_event.unwrap_or_else(|| self.previous_mouse.into()),
        }
    }

    fn activate_game_state(game_loop_data: &mut GameLoopData) -> Result {
        if *game_loop_data.game_state_changed {
            let method_logger = game_loop_data
                .parent_logger
                .new(o!("Method" => "Game::activate_game_state"));
            let current_game_state = game_loop_data
                .game_states
                .get_mut(game_loop_data.current_game_state_length - 1)
                .unwrap();
            debug!(
                method_logger,
                "Calling activate on game state {}",
                current_game_state.name()
            );
            current_game_state
                .activate(game_loop_data.game_ref)
                .context(GameStateError)?;
            *game_loop_data.game_state_changed = false
        }
        Ok(())
    }

    fn update_game_states(game_loop_data: &mut GameLoopData) -> Result<GameStateChange> {
        let game_state_change = 'update: loop {
            for i in 0..game_loop_data.current_game_state_length {
                if i == game_loop_data.current_game_state_length - 1 {
                    break 'update game_loop_data
                        .game_states
                        .get_mut(i)
                        .unwrap()
                        .update(game_loop_data.game_ref)
                        .context(GameStateError)?;
                }

                if let Some(message) = game_loop_data
                    .game_states
                    .get_mut(i)
                    .unwrap()
                    .background_update(game_loop_data.game_ref)
                    .context(GameStateError)?
                {
                    game_loop_data
                        .game_ref
                        .background_update_messages
                        .push(message);
                }
            }
        };
        Ok(game_state_change)
    }

    fn draw_game_states(game_loop_data: &mut GameLoopData) -> Result {
        game_loop_data
            .game_ref
            .root
            .set_default_background(colors::BLACK);
        game_loop_data
            .game_ref
            .root
            .set_default_foreground(colors::WHITE);
        game_loop_data.game_ref.root.clear();
        'draw: loop {
            for i in 0..game_loop_data.current_game_state_length {
                if i == game_loop_data.current_game_state_length - 1 {
                    game_loop_data
                        .game_states
                        .get_mut(i)
                        .unwrap()
                        .draw(game_loop_data.game_ref)
                        .context(GameStateError)?;
                    break 'draw;
                } else {
                    game_loop_data
                        .game_states
                        .get_mut(i)
                        .unwrap()
                        .background_draw(game_loop_data.game_ref)
                        .context(GameStateError)?;
                }
            }
        }
        game_loop_data.game_ref.root.flush();

        Ok(())
    }

    fn deactivate_game_state(
        game_loop_data: &mut GameLoopData,
        game_state_change: &GameStateChange,
    ) -> Result {
        if !game_state_change.is_none() {
            let method_logger = game_loop_data
                .parent_logger
                .new(o!("Method" => "Game::deactivate_game_state"));
            let current_game_state = game_loop_data
                .game_states
                .get_mut(game_loop_data.current_game_state_length - 1)
                .unwrap();
            debug!(
                method_logger,
                "Calling deactivate on game state {}",
                current_game_state.name()
            );
            current_game_state
                .deactivate(game_loop_data.game_ref)
                .context(GameStateError)?;
            *game_loop_data.game_state_changed = true;
        }

        Ok(())
    }

    fn update_game_state(game_loop_data: &mut GameLoopData, game_state_change: GameStateChange) {
        let method_logger = game_loop_data
            .parent_logger
            .new(o!("Method" => "Game::update_game_state"));
        match game_state_change {
            GameStateChange::Replace(next_game_state) => {
                trace!(method_logger, "Game state change: Replace");
                game_loop_data.game_states.clear();
                game_loop_data.game_states.push(next_game_state);
            }
            GameStateChange::Push(next_game_state) => {
                trace!(method_logger, "Game state change: Push");
                game_loop_data.game_states.push(next_game_state)
            }
            GameStateChange::PopPush(next_game_state) => {
                trace!(method_logger, "Game state change: PopPush");
                game_loop_data.game_states.pop();
                game_loop_data.game_states.push(next_game_state)
            }
            GameStateChange::Pop => {
                trace!(method_logger, "Game state change: Pop");
                game_loop_data.game_states.pop();
            }
            GameStateChange::EndGame => {
                trace!(method_logger, "Game state change: EndGame");
                game_loop_data.game_states.clear()
            }
            GameStateChange::None => {
                // trace!(method_logger, "Game state change: None");
            }
        }
    }
}

#[derive(Clone)]
pub struct Input {
    pub raw_events: Vec<input::Event>,
    pub press_key_event: KeyEvent,
    pub release_key_event: KeyEvent,
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
    pub buffer: &'g mut Offscreen,
    pub config: &'g Config,
    pub logger: &'g slog::Logger,
    pub data: &'g mut Data,
    pub game_state_level: usize,
    pub input: Input,
    pub game_data: &'g mut GameData,
    pub background_update_messages: Vec<String>,
}

struct GameLoopData<'d, 'g> {
    parent_logger: &'d slog::Logger,
    game_states: &'d mut Vec<Box<dyn GameState>>,
    current_game_state_length: usize,
    game_state_changed: &'d mut bool,
    game_ref: &'d mut GameRef<'g>,
    //game_state_change: &'d GameStateChange,
    //current_game_state: &'d mut Box<dyn GameState>,
}
