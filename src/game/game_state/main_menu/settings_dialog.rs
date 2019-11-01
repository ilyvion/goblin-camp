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

use crate::data::paths::PathProvider;
use crate::data::settings::{Renderer, Settings};
use crate::game::game_state::{GameState, GameStateChange, GameStateResult, GameStateUpdateResult};
use crate::game::GameRef;
use crate::ui::{MessageBox, Position, Size};
use crate::util::tcod::Chars;
use crate::util::Flip;
use snafu::{ResultExt, Snafu};
use std::borrow::Cow;
use std::path::Path;
use tcod::console::Root;
use tcod::input::KeyCode;
use tcod::{colors, BackgroundFlag, Console, TextAlignment};

#[derive(Debug, Snafu)]
pub enum Error {
    SettingsSave {
        source: crate::data::settings::Error,
    },
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct SettingsDialog {
    fields: [SettingField; 2],
    focused_field: usize,
    original_settings: Option<Settings>,
    message_box: bool,
}

impl SettingsDialog {
    const WIDTH: i32 = 40;
    const HEIGHT: i32 = 28;

    pub fn game_state_change(_: &mut GameRef) -> GameStateChange {
        GameStateChange::Push(Self::game_state())
    }

    pub fn game_state() -> Box<dyn GameState> {
        Box::new(Self {
            fields: [
                SettingField {
                    label: "Resolution (width)",
                    value: Default::default(),
                    invalid: false,
                },
                SettingField {
                    label: "Resolution (height)",
                    value: Default::default(),
                    invalid: false,
                },
            ],
            focused_field: 0,
            original_settings: None,
            message_box: false,
        })
    }

    pub fn save_settings<P: AsRef<Path>>(&mut self, settings: &Settings, path: P) -> Result {
        settings.save(path).context(SettingsSave)?;

        Ok(())
    }
}

impl GameState for SettingsDialog {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed("Settings dialog")
    }

    fn activate(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        if !self.message_box {
            self.original_settings = Some(game_ref.data.settings.clone());
            self.fields[0].value = game_ref.data.settings.display.resolution.width.to_string();
            self.fields[0].invalid = false;
            self.fields[1].value = game_ref.data.settings.display.resolution.height.to_string();
            self.fields[1].invalid = false;
        } else {
            self.message_box = false;
        }

        Ok(())
    }

    fn update(&mut self, game_ref: &mut GameRef) -> GameStateUpdateResult {
        if game_ref.input.key_event.raw.code == KeyCode::Escape {
            game_ref
                .data
                .settings
                .restore_from(self.original_settings.take().unwrap());
            return Ok(GameStateChange::Pop);
        } else if game_ref.input.key_event.raw.code == KeyCode::Enter {
            if self.fields[0].invalid || self.fields[1].invalid {
                self.message_box = true;
                return Ok(GameStateChange::Push(MessageBox::game_state(
                    game_ref,
                    "Invalid value(s) for resolution",
                    "Understood",
                    Box::new(|| GameStateChange::Pop),
                    None,
                    None,
                )));
            }

            self.save_settings(&game_ref.data.settings, game_ref.data.paths.settings_file())?;
            return Ok(GameStateChange::Pop);
        } else {
            let field_value = &mut self.fields[self.focused_field].value;
            let field_invalid = &mut self.fields[self.focused_field].invalid;

            let key = game_ref.input.key_event.raw.printable;
            let code = game_ref.input.key_event.raw.code;
            let mut field_updated = false;
            if key >= '0' && key <= '9' && field_value.len() < (Self::WIDTH - 7) as usize {
                field_value.push(key);
                field_updated = true;
            } else if code == KeyCode::Backspace {
                field_value.pop();
                field_updated = true;
            }
            if field_updated {
                if let Ok(value) = field_value.parse() {
                    *field_invalid = false;
                    if self.focused_field == 0 {
                        game_ref.data.settings.display.resolution.width = value;
                    } else {
                        game_ref.data.settings.display.resolution.height = value;
                    }
                } else {
                    *field_invalid = true;
                }
            }
        }

        let dialog_position = Position::new(
            game_ref.root.width() / 2 - (Self::WIDTH / 2),
            game_ref.root.height() / 2 - (Self::HEIGHT / 2),
        );

        let mouse_event = game_ref.input.mouse_event;
        if mouse_event.clicked
            && (dialog_position + Size::new(Self::WIDTH, Self::HEIGHT))
                .contains_position(mouse_event.character_position)
        {
            let internal_position = mouse_event.character_position - dialog_position;

            match internal_position.y {
                4 | 5 => {
                    self.focused_field = 0;
                }
                6 | 7 => {
                    self.focused_field = 1;
                }
                9 => {
                    // Fullscreen
                    game_ref.data.settings.display.fullscreen.flip();
                }
                11 => {
                    // Tutorial
                    game_ref.data.settings.tutorial.flip();
                }
                13 => {
                    // Translucent UI
                    game_ref.data.settings.translucent_ui.flip();
                }
                15 => {
                    // Compress saves
                    game_ref.data.settings.compress_saves.flip();
                }
                17 => {
                    // Auto save
                    game_ref.data.settings.auto_save.flip();
                }
                19 => {
                    // Pause on danger
                    game_ref.data.settings.pause_on_danger.flip();
                }
                26 => {
                    // Use tile set
                    game_ref.data.settings.use_tile_set.flip();
                }
                22..=24 => {
                    game_ref.data.settings.renderer =
                        Renderer::from_index(2 - (24 - internal_position.y) as usize).unwrap();
                }
                _ => (),
            }
        }

        Ok(GameStateChange::None)
    }

    fn draw(&mut self, game_ref: &mut GameRef) -> GameStateResult {
        game_ref.root.set_alignment(TextAlignment::Left);

        let x = game_ref.root.width() / 2 - (Self::WIDTH / 2);
        let y = game_ref.root.height() / 2 - (Self::HEIGHT / 2);

        game_ref.root.set_default_foreground(colors::WHITE);
        game_ref.root.set_default_background(colors::BLACK);

        game_ref.root.print_frame(
            x,
            y,
            Self::WIDTH,
            Self::HEIGHT,
            true,
            BackgroundFlag::Set,
            Some("Settings"),
        );
        game_ref
            .root
            .print(x + 1, y + 1, "ENTER to save changes, ESC to discard.");

        let mut current_y = y + 3;
        for (i, field) in self.fields.iter().enumerate() {
            if self.focused_field == i {
                game_ref.root.set_default_foreground(colors::GREEN);
            }
            game_ref.root.print(x + 1, current_y, field.label);

            if field.invalid {
                game_ref.root.set_default_background(colors::DARKER_RED);
            } else {
                game_ref.root.set_default_background(colors::DARK_GREY);
            }
            game_ref.root.set_default_foreground(colors::WHITE);
            game_ref.root.rect(
                x + 3,
                current_y + 1,
                Self::WIDTH - 7,
                1,
                true,
                BackgroundFlag::Default,
            );
            game_ref.root.print(x + 3, current_y + 1, &field.value);
            game_ref.root.set_default_background(colors::BLACK);

            current_y += 3;
        }

        let print_bool_setting = |root: &mut Root, value: bool, y: i32, text: &str| {
            root.set_default_foreground(if value { colors::GREEN } else { colors::GREY });

            root.put_char(
                x + 1,
                y,
                if !value {
                    Chars::CheckboxUnset.into()
                } else {
                    Chars::CheckboxSet.into()
                },
                BackgroundFlag::Default,
            );
            root.print(x + 3, y, text);
        };

        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.display.fullscreen,
            current_y,
            "Fullscreen mode",
        );
        current_y += 2;
        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.tutorial,
            current_y,
            "Tutorial",
        );
        current_y += 2;
        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.translucent_ui,
            current_y,
            "Translucent UI",
        );
        current_y += 2;
        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.compress_saves,
            current_y,
            "Compress saves",
        );
        current_y += 2;
        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.auto_save,
            current_y,
            "Auto save",
        );
        current_y += 2;
        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.pause_on_danger,
            current_y,
            "Pause on danger",
        );
        current_y += 2;

        game_ref.root.set_default_foreground(colors::WHITE);
        game_ref.root.print(x + 1, current_y, "Renderer");

        for (i, renderer) in Renderer::all().enumerate() {
            if renderer == game_ref.data.settings.renderer {
                game_ref.root.set_default_foreground(colors::GREEN);
            } else {
                game_ref.root.set_default_foreground(colors::GREY);
            }
            game_ref
                .root
                .print(x + 3, current_y + i as i32 + 1, renderer.label());
        }

        current_y += 5;
        print_bool_setting(
            game_ref.root,
            game_ref.data.settings.use_tile_set,
            current_y,
            "Use tile set",
        );

        Ok(())
    }
}

struct SettingField {
    label: &'static str,
    value: String,
    invalid: bool,
}
