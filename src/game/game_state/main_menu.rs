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

use crate::game::game_state::game::ConfirmNewGame;
use crate::game::game_state::GameStateUpdateResult;
use crate::game::game_state::{GameState, GameStateChange, GameStateResult};
use crate::game::{Game, GameRef};
use derivative::Derivative;
use slog::{debug, o};
use std::borrow::Cow;
use tcod::colors::{BLACK, CELADON, GREY, WHITE};
use tcod::console::BackgroundFlag::{Default as BackgroundDefault, Set};
use tcod::console::{Console, Root};
use tcod::input::Event;
use tcod::TextAlignment;

pub struct MainMenu {
    logger: Option<slog::Logger>,
    render_data: Option<RenderData>,
}

impl MainMenu {
    const ENTRIES: [MainMenuEntry; 9] = [
        MainMenuEntry {
            label: "New Game",
            shortcut: 'n',
            active: ActiveState::Always,
            new_state: ConfirmNewGame::game_state,
        },
        MainMenuEntry {
            label: "Continue",
            shortcut: 'c',
            active: ActiveState::IfRunning,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Load",
            shortcut: 'l',
            active: ActiveState::HasSaves,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Save",
            shortcut: 's',
            active: ActiveState::IfRunning,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Settings",
            shortcut: 'o',
            active: ActiveState::Never,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Keys",
            shortcut: 'k',
            active: ActiveState::Never,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Mods",
            shortcut: 'm',
            active: ActiveState::Never,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Tile sets",
            shortcut: 't',
            active: ActiveState::Never,
            new_state: MainMenu::quit_game_state,
        },
        MainMenuEntry {
            label: "Exit",
            shortcut: 'q',
            active: ActiveState::Always,
            new_state: MainMenu::quit_game_state,
        },
    ];

    const WIDTH: i32 = 20;

    pub fn game_state() -> Box<dyn GameState> {
        Box::new(MainMenu {
            logger: None,
            render_data: None,
        })
    }

    fn quit_game_state() -> GameStateChange {
        GameStateChange::EndGame
    }

    fn render(&mut self, game_ref: &mut GameRef, background: bool) -> GameStateResult {
        let render_data = self.render_data.as_mut().unwrap();

        Self::render_menu(game_ref.root, render_data.clone());
        Self::render_menu_entries(game_ref, render_data.clone());

        if !background && !render_data.render_credits_done {
            render_data.render_credits_done =
                game_ref
                    .root
                    .render_credits(render_data.edge_x + 5, render_data.edge_y + 25, true);
        }

        Ok(())
    }

    fn render_menu(root: &mut Root, render_data: RenderData) {
        root.set_default_foreground(WHITE);
        root.set_default_background(BLACK);

        root.print_frame(
            render_data.edge_x,
            render_data.edge_y,
            Self::WIDTH,
            render_data.height,
            true,
            BackgroundDefault,
            Some("Main Menu"),
        );
        root.set_alignment(TextAlignment::Center);
        root.set_background_flag(Set);

        root.set_default_foreground(CELADON);
        root.print(
            render_data.edge_x + Self::WIDTH / 2,
            render_data.edge_y - 3,
            format!("{} {}", Game::NAME, Game::VERSION),
        );
    }

    fn render_menu_entries(game_ref: &mut GameRef, render_data: RenderData) {
        game_ref.root.set_default_foreground(WHITE);

        for (i, entry) in MainMenu::ENTRIES.iter().enumerate() {
            if render_data
                .selected
                .map_or(false, |selected_entry| selected_entry == entry)
            {
                game_ref.root.set_default_foreground(BLACK);
                game_ref.root.set_default_background(WHITE);
            } else {
                game_ref.root.set_default_foreground(WHITE);
                game_ref.root.set_default_background(BLACK);
            }

            if !entry.is_active(game_ref) {
                game_ref.root.set_default_foreground(GREY);
            }

            game_ref.root.print(
                render_data.edge_x + Self::WIDTH / 2,
                render_data.edge_y + ((i + 1) * 2) as i32,
                entry.label,
            );
        }
    }

    fn handle_input(
        game_ref: &mut GameRef,
        render_data: RenderData,
        input_data: &mut InputData,
    ) -> Option<GameStateUpdateResult> {
        let method_logger = render_data
            .logger
            .new(o!("Method" => "MainMenu::handle_input"));
        // TODO: Rewrite to use the Input data directly
        if let Some(raw_event) = game_ref.input.raw_events.first().cloned() {
            match raw_event {
                Event::Key(key) => {
                    for entry in MainMenu::ENTRIES.iter() {
                        if key.printable == entry.shortcut && entry.is_active(game_ref) {
                            debug!(method_logger, "Entry chosen by key"; "entry" => entry.label, "key" => key.printable);
                            return Some(Ok((entry.new_state)()));
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    input_data.selected = if mouse.cx > render_data.edge_x as isize
                        && mouse.cx < (render_data.edge_x + Self::WIDTH) as isize
                    {
                        let selected_line = (mouse.cy - (render_data.edge_y + 2) as isize) as usize;
                        let selected_index = selected_line / 2;
                        if selected_line % 2 == 0 && selected_index < MainMenu::ENTRIES.len() {
                            Some(&MainMenu::ENTRIES[selected_index])
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    if mouse.lbutton {
                        input_data.l_button_down = true
                    } else if !mouse.lbutton && input_data.l_button_down {
                        input_data.l_button_down = false;
                        if let Some(selected) = input_data.selected {
                            if selected.is_active(game_ref) {
                                let mouse_coordinates = format!("({}, {})", mouse.cx, mouse.cy);
                                debug!(method_logger, "Entry chosen by mouse"; "entry" => selected.label, "position" => mouse_coordinates);
                                return Some(Ok((selected.new_state)()));
                            }
                        }
                    }
                }
            }
        }

        None
    }
}

impl GameState for MainMenu {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Main menu")
    }

    fn activate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        if self.logger.is_none() {
            let logger = game_ref.logger.new(o!("GameState" => "MainMenu"));
            self.logger = Some(logger);

            let root = &game_ref.root;
            let height: i32 = (MainMenu::ENTRIES.len() * 2 + 2) as i32;
            let edge_x = root.width() / 2 - Self::WIDTH / 2;
            let edge_y = root.height() / 2 - height / 2;

            let logger = self.logger.as_ref().unwrap().clone();
            let render_data = RenderData {
                edge_x,
                edge_y,
                height,
                render_credits_done: false,
                l_button_down: false,
                selected: None,
                logger,
            };
            self.render_data = Some(render_data);
        } else {
            let render_data = self.render_data.as_mut().unwrap();
            render_data.render_credits_done = false;
            render_data.l_button_down = false;
            render_data.selected = None;
        }

        Ok(())
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        let render_data = self.render_data.as_mut().unwrap();

        if true {
            let mut input_data = InputData {
                l_button_down: render_data.l_button_down,
                selected: render_data.selected,
            };
            let result = Self::handle_input(game_ref, render_data.clone(), &mut input_data);
            if let Some(result) = result {
                return result;
            }
            let InputData {
                l_button_down: l_button_down_new,
                selected: selected_new,
            } = input_data;
            render_data.l_button_down = l_button_down_new;
            render_data.selected = selected_new;
        }

        if game_ref.root.window_closed() {
            Ok(GameStateChange::EndGame)
        } else {
            Ok(GameStateChange::NoOp)
        }
    }

    fn background_draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        self.render(game_ref, true)
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        self.render(game_ref, false)
    }
}

#[derive(Derivative, Clone)]
#[derivative(Debug)]
struct RenderData {
    height: i32,
    edge_x: i32,
    edge_y: i32,
    render_credits_done: bool,
    l_button_down: bool,
    #[derivative(Debug = "ignore")]
    selected: Option<&'static MainMenuEntry>,
    #[derivative(Debug = "ignore")]
    logger: slog::Logger,
}

#[derive(Copy, Clone)]
struct InputData {
    l_button_down: bool,
    selected: Option<&'static MainMenuEntry>,
}

#[derive(Debug, Eq, PartialEq)]
enum ActiveState {
    Always,
    IfRunning,
    HasSaves,
    Never,
}

#[derive(Debug, Eq, PartialEq)]
struct MainMenuEntry {
    label: &'static str,
    shortcut: char,
    active: ActiveState,
    new_state: fn() -> GameStateChange,
}

impl MainMenuEntry {
    pub fn is_active(&self, game_ref: &mut GameRef) -> bool {
        // TODO: Proper logic for deciding whether this entry is active or not
        match self.active {
            ActiveState::Always => true,
            ActiveState::IfRunning => game_ref.is_running,
            _ => false,
        }
    }
}
