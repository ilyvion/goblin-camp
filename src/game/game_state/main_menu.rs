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

use crate::game::game_state::{GameState, GameStateChange};
use crate::game::game_state::Result;
use tcod::TextAlignment;
use tcod::colors::{WHITE, BLACK, CELADON, GREY};
use tcod::console::BackgroundFlag::{Default as BackgroundDefault, Set};
use crate::game::{Game, GameRef};
use tcod::input::{Event, KEY_RELEASE, MOUSE};
use tcod::console::{Console, Root};

pub struct MainMenu;

impl MainMenu {
    const ENTRIES: [MainMenuEntry; 9] = [
        MainMenuEntry { label: "New Game", shortcut: 'n', active: ActiveState::Always, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Continue", shortcut: 'c', active: ActiveState::IfRunning, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Load", shortcut: 'l', active: ActiveState::HasSaves, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Save", shortcut: 's', active: ActiveState::IfRunning, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Settings", shortcut: 'o', active: ActiveState::Always, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Keys", shortcut: 'k', active: ActiveState::Always, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Mods", shortcut: 'm', active: ActiveState::Always, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Tile sets", shortcut: 't', active: ActiveState::Always, new_state: MainMenu::quit_game_state },
        MainMenuEntry { label: "Exit", shortcut: 'q', active: ActiveState::Always, new_state: MainMenu::quit_game_state },
    ];

    const WIDTH: i32 = 20;

    pub fn game_state() -> Box<dyn GameState> { Box::new(MainMenu) }

    fn quit_game_state() -> GameStateChange { GameStateChange::EndGame }

    fn render_menu(root: &mut Root, render_data: RenderData) {
        root.set_default_foreground(WHITE);
        root.set_default_background(BLACK);
        root.clear();
        root.print_frame(render_data.edge_x, render_data.edge_y, Self::WIDTH, render_data.height, true, BackgroundDefault, Some("Main Menu"));
        root.set_alignment(TextAlignment::Center);
        root.set_background_flag(Set);

        root.set_default_foreground(CELADON);
        root.print(render_data.edge_x + Self::WIDTH / 2, render_data.edge_y - 3, format!("Goblin Camp {}", Game::VERSION));
    }

    fn render_menu_entries(root: &mut Root, render_data: RenderData) {
        root.set_default_foreground(WHITE);

        for (i, entry) in MainMenu::ENTRIES.iter().enumerate() {
            if render_data.selected.map_or(false, |selected_entry_shortcut| selected_entry_shortcut == entry.shortcut) {
                root.set_default_foreground(BLACK);
                root.set_default_background(WHITE);
            } else {
                root.set_default_foreground(WHITE);
                root.set_default_background(BLACK);
            }

            if !entry.is_active() {
                root.set_default_foreground(GREY);
            }

            root.print(render_data.edge_x + Self::WIDTH / 2, render_data.edge_y + ((i + 1) * 2) as i32, entry.label);
        }
    }

    fn handle_input(render_data: RenderData, input_data: &mut InputData) -> Option<Result<GameStateChange>> {
        //let mut selected = selected;
        for (flags, event) in tcod::input::events() {
            if flags.intersects(KEY_RELEASE) {
                if let Event::Key(key) = event {
                    for entry in MainMenu::ENTRIES.iter() {
                        if key.printable == entry.shortcut && entry.is_active() {
                            return Some(Ok((entry.new_state)()));
                        }
                    }
                }
            } else if flags.intersects(MOUSE) {
                if let Event::Mouse(mouse) = event {
                    input_data.selected = if mouse.cx > render_data.edge_x as isize && mouse.cx < (render_data.edge_x + Self::WIDTH) as isize {
                        let selected_index: usize = (mouse.cy - (render_data.edge_y + 2) as isize) as usize / 2;
                        if mouse.cy % 2 == 0 && selected_index < MainMenu::ENTRIES.len() {
                            Some(&MainMenu::ENTRIES[selected_index])
                        } else { None }
                    } else { None };

                    if mouse.lbutton {
                        input_data.l_button_down = true
                    } else if !mouse.lbutton && input_data.l_button_down {
                        input_data.l_button_down = false;
                        if let Some(selected) = input_data.selected {
                            if selected.is_active() {
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
    fn handle(&mut self, game_ref: GameRef) -> Result<GameStateChange> {
        let root = game_ref.root;

        let height: i32 = (MainMenu::ENTRIES.len() * 2 + 2) as i32;
        let edge_x = root.width() / 2 - Self::WIDTH / 2;
        let edge_y = root.height() / 2 - height / 2;
        let mut selected: Option<&MainMenuEntry> = None;
        let mut render_credits_done = false;
        let mut l_button_down = false;

        let mut render_data = RenderData {
            edge_x,
            edge_y,
            height,
            selected: None,
        };

        loop {
            render_data.selected = selected.map(|s| s.shortcut);

            Self::render_menu(root, render_data);
            Self::render_menu_entries(root, render_data);

            if !render_credits_done {
                render_credits_done = root.render_credits(edge_x + 5, edge_y + 25, true);
            }

            root.flush();

            let mut input_data = InputData {
                l_button_down,
                selected,
            };
            let result = Self::handle_input(render_data, &mut input_data);
            if let Some(result) = result {
                return result;
            }
            let InputData { l_button_down: l_button_down_new, selected: selected_new } = input_data;
            l_button_down = l_button_down_new;
            selected = selected_new;

            if root.window_closed() {
                return Ok(GameStateChange::EndGame);
            }
        }
    }
}

#[derive(Copy, Clone)]
struct RenderData {
    height: i32,
    edge_x: i32,
    edge_y: i32,
    selected: Option<char>,
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
    _Never,
}

#[derive(Debug, Eq, PartialEq)]
struct MainMenuEntry {
    label: &'static str,
    shortcut: char,
    active: ActiveState,
    new_state: fn() -> GameStateChange,
}

impl MainMenuEntry {
    pub fn is_active(&self) -> bool {
        // TODO: Proper logic for deciding whether to show this entry or not
        match self.active {
            ActiveState::Always => true,
            _ => false
        }
    }
}
